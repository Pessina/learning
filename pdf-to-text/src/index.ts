import {
  TextractClient,
  AnalyzeDocumentCommand,
  Block,
  FeatureType,
  Relationship,
} from "@aws-sdk/client-textract";
import { readFileSync } from "fs";

const client = new TextractClient({ region: "us-west-2" });

async function extractBookContent(filePath: string): Promise<void> {
  try {
    const fileContent = readFileSync(filePath);
    const command = new AnalyzeDocumentCommand({
      Document: { Bytes: fileContent },
      FeatureTypes: [FeatureType.TABLES, FeatureType.LAYOUT],
    });
    const response = await client.send(command);

    const blocks: Block[] = response.Blocks || [];
    const layoutBlocks = extractLayoutBlocks(blocks);
    const tableBlocks = extractTableBlocks(blocks);

    const extractedText = combineLayoutAndTableContent(
      layoutBlocks,
      tableBlocks,
      blocks
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
    const pageText =
      getTextFromBlocks(block.Relationships ?? [], rootBlocks) + "\n";
    combinedText += pageText + "\n\n";
  });

  tableBlocks.forEach((block) => {
    combinedText +=
      "Table:\n" + printTableAsCSV(block ?? [], rootBlocks) + "\n\n";
  });

  return combinedText;
}

function getTextFromBlocks(
  relationships: Relationship[],
  blocks: Block[]
): string {
  let combinedText = "";

  relationships.forEach((relationship) => {
    if (relationship.Type === "CHILD") {
      relationship.Ids?.forEach((childId) => {
        const block = blocks.find((b) => b.Id === childId);
        if (!block) {
          return;
        }

        if (
          block.Relationships &&
          block.Relationships.some((rel) => rel.Type === "CHILD")
        ) {
          combinedText +=
            getTextFromBlocks(
              block.Relationships.filter((rel) => rel.Type === "CHILD"),
              blocks
            ) + " ";
        } else {
          combinedText += block.Text + " " || "";
        }
      });
    }
  });

  return combinedText.trim();
}

function printTableAsCSV(tableBlock: Block, blocks: Block[]): string {
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

      // Ensure rows array has enough subarrays to include this cell
      while (rows.length <= rowIndex) {
        rows.push([]);
      }

      // Get the text from the cell
      const cellText = getTextFromBlocks(cellBlock.Relationships ?? [], blocks);

      // Place the cell text in the correct column
      rows[rowIndex][columnIndex] = cellText;
    }
  });

  // Convert rows to CSV format
  const csvContent = rows
    .map((row) => {
      // Fill undefined cells with empty strings and join with commas
      return row.map((cell) => cell ?? "").join(",");
    })
    .join("\n");

  return csvContent;
}

const bookFilePath = "./books/page.pdf";
extractBookContent(bookFilePath);
