use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Notebook {
    cells: Vec<Cell>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cell_type")]
#[allow(non_camel_case_types)]
enum Cell {
    markdown { source: Vec<String> },
    code { source: Vec<String> },
}

pub fn convert_string_to_markdown(file_content: String) -> Result<String> {
    let notebook: Notebook = serde_json::from_str(&file_content)?;

    let mut markdown_content = String::new();
    for cell in notebook.cells {
        match cell {
            Cell::markdown { source } => {
                markdown_content.push_str(&source.join("\n"));
                markdown_content.push('\n');
            }
            Cell::code { source } => {
                markdown_content.push_str("```python\n");
                markdown_content.push_str(&source.join(""));
                markdown_content.push_str("\n```\n");
            }
        }
    }

    Ok(markdown_content)
}
