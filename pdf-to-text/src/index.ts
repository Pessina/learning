import {
  TextractClient,
  Block,
  FeatureType,
  Relationship,
  StartDocumentAnalysisCommand,
  GetDocumentAnalysisCommand,
} from "@aws-sdk/client-textract";
import { readFileSync } from "fs";
import { S3Client, PutObjectCommand } from "@aws-sdk/client-s3";

const REGION = "us-east-1";

const client = new TextractClient({ region: REGION });

async function extractBookContent(filePath: string): Promise<void> {
  try {
    const BUCKET_NAME = "pessina-textract-docs";
    const KEY = "page-range.pdf";

    const fileContent = readFileSync(filePath);

    const s3: S3Client = new S3Client({ region: REGION });
    await s3.send(
      new PutObjectCommand({
        Bucket: BUCKET_NAME,
        Key: KEY,
        Body: fileContent,
      })
    );

    const command = new StartDocumentAnalysisCommand({
      DocumentLocation: {
        S3Object: {
          Bucket: BUCKET_NAME,
          Name: KEY,
        },
      },
      FeatureTypes: [FeatureType.TABLES, FeatureType.LAYOUT],
    });

    const { JobId } = await client.send(command);

    let nextToken: string | undefined = "initial";
    let pages: Block[] = [];

    do {
      await new Promise<void>((resolve) => setTimeout(resolve, 5000));

      const getResultCommand: GetDocumentAnalysisCommand =
        new GetDocumentAnalysisCommand({
          JobId,
          MaxResults: 1000,
          ...(nextToken !== "initial" ? { NextToken: nextToken } : {}),
        });

      const response = await client.send(getResultCommand);

      if (response.JobStatus === "IN_PROGRESS") {
        continue;
      } else if (response.JobStatus === "SUCCEEDED" && response.Blocks) {
        pages = pages.concat(response.Blocks);
        nextToken = response.NextToken;
      } else if (response.JobStatus === "FAILED") {
        throw new Error(`Document analysis failed: ${response.StatusMessage}`);
      }
    } while (nextToken);

    console.log(pages);

    const layoutBlocks = extractLayoutBlocks(pages);
    const tableBlocks = extractTableBlocks(pages);

    const extractedText = combineLayoutAndTableContent(
      layoutBlocks,
      tableBlocks,
      pages
    );

    console.log(extractedText);
  } catch (error) {
    console.error("Error:", error);
  }
}

function extractLayoutBlocks(blocks: Block[]): Block[] {
  const layoutBlocks: Block[] = [];
  blocks.forEach((block) => {
    if (block.BlockType?.startsWith("LAYOUT_")) {
      layoutBlocks.push(block);
    }
  });
  return layoutBlocks;
}

function extractTableBlocks(blocks: Block[]): Block[] {
  const tableBlocks: Block[] = [];
  blocks.forEach((block) => {
    if (block.BlockType === "TABLE") {
      tableBlocks.push(block);
    }
  });
  return tableBlocks;
}

function combineLayoutAndTableContent(
  layoutBlocks: Block[],
  tableBlocks: Block[],
  rootBlocks: Block[]
): string {
  let combinedText = "";

  layoutBlocks.forEach((block) => {
    if (block.BlockType === "LAYOUT_TABLE" && tableBlocks.length > 0) {
      const table = tableBlocks.shift();
      if (table) {
        combinedText +=
          printTableAsCSV(table, rootBlocks, tableBlocks) + "\n\n";
      }
    } else {
      const pageText =
        getTextFromBlocks(block.Relationships ?? [], rootBlocks, tableBlocks) +
        "\n";
      combinedText += pageText + "\n";
    }
  });

  return combinedText;
}

function getTextFromBlocks(
  relationships: Relationship[],
  blocks: Block[],
  tableBlocks: Block[]
): string {
  let combinedText = "";

  relationships.forEach((relationship) => {
    if (relationship.Type === "CHILD") {
      relationship.Ids?.forEach((childId) => {
        const block = blocks.find((b) => b.Id === childId);
        if (!block) {
          return;
        } else if (
          block.Relationships &&
          block.Relationships.some((rel) => rel.Type === "CHILD")
        ) {
          combinedText +=
            getTextFromBlocks(
              block.Relationships.filter((rel) => rel.Type === "CHILD"),
              blocks,
              tableBlocks
            ) + " ";
        } else {
          combinedText += block.Text + " " || "";
        }
      });
    }
  });

  return combinedText.trim();
}

function printTableAsCSV(
  tableBlock: Block,
  blocks: Block[],
  tableBlocks: Block[]
): string {
  if (tableBlock.BlockType !== "TABLE") {
    return "Invalid block type for table.";
  }

  const rows: string[][] = [];
  const childIds =
    tableBlock.Relationships?.find((rel) => rel.Type === "CHILD")?.Ids ?? [];

  childIds.forEach((cellId) => {
    const cellBlock = blocks.find((block) => block.Id === cellId);
    if (cellBlock && cellBlock.BlockType === "CELL") {
      const rowIndex = cellBlock.RowIndex ?? 0;
      const columnIndex = cellBlock.ColumnIndex ?? 0;

      while (rows.length <= rowIndex) {
        rows.push([]);
      }

      const cellText = getTextFromBlocks(
        cellBlock.Relationships ?? [],
        blocks,
        tableBlocks
      );

      rows[rowIndex][columnIndex] = cellText;
    }
  });

  const csvContent = rows
    .map((row) => {
      return row.map((cell) => cell ?? "").join(",");
    })
    .join("\n");

  return csvContent;
}

const bookFilePath = "./books/page-range.pdf";
extractBookContent(bookFilePath);
