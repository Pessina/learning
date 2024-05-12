import {
  TextractClient,
  AnalyzeDocumentCommand,
  AnalyzeDocumentCommandInput,
  Block,
  FeatureType,
  Relationship,
} from "@aws-sdk/client-textract";
import { readFileSync } from "fs";

const client = new TextractClient({ region: "us-west-2" });

async function extractBookContent(filePath: string): Promise<void> {
  try {
    const fileContent = readFileSync(filePath);
    const params: AnalyzeDocumentCommandInput = {
      Document: { Bytes: fileContent },
      FeatureTypes: [FeatureType.TABLES, FeatureType.LAYOUT],
    };
    const command = new AnalyzeDocumentCommand(params);
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

  console.log(JSON.stringify(layoutBlocks[0]));
  console.log(JSON.stringify(layoutBlocks[1]));
  console.log(JSON.stringify(layoutBlocks[2]));
  // Iterate over the PAGE blocks
  layoutBlocks.forEach((block) => {
    const pageText =
      "Layout: " +
      getTextFromBlocksById(block.Relationships ?? [], rootBlocks) +
      "\n\n";
    combinedText += pageText + "\n\n";
  });

  // Process table blocks
  tableBlocks.forEach((block) => {
    combinedText += "Table:\n" + block.Text + "\n\n";
  });

  return combinedText;
}

function getTextFromBlocksById(
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

        // If the block has a relationship of type 'CHILD', recursively get text from child blocks
        if (
          block.Relationships &&
          block.Relationships.some((rel) => rel.Type === "CHILD")
        ) {
          combinedText +=
            getTextFromBlocksByRelationship(
              block.Relationships.filter((rel) => rel.Type === "CHILD"),
              blocks
            ) + " ";
        } else {
          // If it's a final node, return its text
          combinedText += block.Text || "";
        }
      });
    }
  });

  return combinedText.trim();
}

function getTextFromBlocksByRelationship(
  relationships: Relationship[],
  blocks: Block[]
): string {
  let combinedText = "";

  relationships.forEach((relationship) => {
    relationship.Ids?.forEach((childId) => {
      const block = blocks.find((b) => b.Id === childId);
      if (!block) {
        return;
      }

      // If the block has a relationship of type 'CHILD', recursively get text from child blocks
      if (
        block.Relationships &&
        block.Relationships.some((rel) => rel.Type === "CHILD")
      ) {
        combinedText +=
          getTextFromBlocksByRelationship(
            block.Relationships.filter((rel) => rel.Type === "CHILD"),
            blocks
          ) + " ";
      } else {
        // If it's a final node, return its text
        combinedText += block.Text || "";
      }
    });
  });

  return combinedText.trim();
}

const bookFilePath = "./books/page.pdf";
extractBookContent(bookFilePath);
