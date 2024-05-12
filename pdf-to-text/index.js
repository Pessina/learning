const {
  TextractClient,
  AnalyzeDocumentCommand,
} = require("@aws-sdk/client-textract");
const { readFileSync } = require("fs");

const client = new TextractClient({ region: "us-west-2" });

async function extractBookContent(filePath) {
  try {
    const fileContent = readFileSync(filePath);

    const params = {
      Document: {
        Bytes: fileContent,
      },
      FeatureTypes: ["TABLES", "FORMS"],
    };

    const command = new AnalyzeDocumentCommand(params);
    const response = await client.send(command);

    const blocks = response.Blocks || [];
    let extractedText = "";
    let currentTable = [];

    blocks.forEach((block) => {
      if (block.BlockType === "LINE") {
        extractedText += block.Text + "\n";
      } else if (block.BlockType === "TABLE") {
        if (currentTable.length > 0) {
          extractedText +=
            currentTable.map((row) => row.join(",")).join("\n") + "\n\n";
          currentTable = [];
        }
      } else if (block.BlockType === "CELL") {
        const rowIndex = block.RowIndex - 1;
        const columnIndex = block.ColumnIndex - 1;
        if (!currentTable[rowIndex]) {
          currentTable[rowIndex] = [];
        }
        currentTable[rowIndex][columnIndex] = getCellText(block, blocks);
      }
    });

    if (currentTable.length > 0) {
      extractedText +=
        currentTable.map((row) => row.join(",")).join("\n") + "\n\n";
    }

    console.log(extractedText);
  } catch (error) {
    console.error("Error:", error);
  }
}

function getCellText(cell, blocks) {
  const relationships = cell.Relationships || [];
  const textBlocks = relationships
    .filter((r) => r.Type === "CHILD")
    .flatMap((r) =>
      r.Ids.map((id) => blocks.find((b) => b.Id === id)).filter(Boolean)
    );
  return textBlocks.map((b) => b.Text).join(" ");
}

const bookFilePath = "./books/page.pdf";
extractBookContent(bookFilePath);
