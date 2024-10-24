/// Original author of this code is [Nathan Ringo](https://github.com/remexre)
/// Source: https://github.com/acmumn/mentoring/blob/master/web-client/src/view/markdown.rs
use pulldown_cmark::{Alignment, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use yew::virtual_dom::{VNode, VTag, VText};
use yew::Html;

struct TableContext {
    next_cell_index: usize,
    in_head: bool,
    has_body: bool,
    alignment: Vec<Alignment>,
}

struct TagWriter {
    root_children: Vec<VNode>,
    spine: Vec<VTag>,
    table_ctx: Option<TableContext>,
}

impl TagWriter {
    fn new() -> Self {
        Self {
            root_children: vec![],
            spine: vec![],
            table_ctx: None,
        }
    }

    fn finish(mut self) -> VNode {
        assert!(
            self.spine.is_empty(),
            "expected all nested elements to be closed"
        );
        if self.root_children.len() == 1 {
            self.root_children.pop().unwrap()
        } else {
            self.root_children.into_iter().collect()
        }
    }

    fn add_child(&mut self, child: VNode) {
        if let Some(host) = self.spine.last_mut() {
            host.add_child(child);
        } else {
            self.root_children.push(child);
        }
    }

    fn pop_spine(&mut self) {
        let top = self.spine.pop().expect("an element to close");
        self.add_child(top.into());
    }

    fn get_table_ctx(&mut self) -> &mut TableContext {
        self.table_ctx.as_mut().expect("a table context")
    }

    fn open_table_ctx(&mut self, alignment: Vec<Alignment>) {
        assert!(self.table_ctx.is_none(), "nested tables not supported");
        self.table_ctx = Some(TableContext {
            next_cell_index: 0,
            in_head: false,
            has_body: false,
            alignment,
        });
    }

    fn close_table_ctx(&mut self) -> TableContext {
        self.table_ctx.take().expect("expected to be in a table")
    }

    fn start_tag(&mut self, tag: Tag) {
        let wrapper = match tag {
            Tag::Paragraph => {
                let mut el = VTag::new("p");
                el.add_attribute("class", "mb-2");
                el
            }
            Tag::Heading { level, .. } => {
                let mut el = VTag::new(level.to_string());
                match level {
                    HeadingLevel::H1 => el.add_attribute("class", "my-4 text-5xl font-extrabold"),
                    HeadingLevel::H2 => el.add_attribute("class", "my-4 text-4xl font-bold"),
                    HeadingLevel::H3 => el.add_attribute("class", "my-4 text-3xl font-bold"),
                    HeadingLevel::H4 => el.add_attribute("class", "my-4 text-2xl font-bold"),
                    HeadingLevel::H5 => el.add_attribute("class", "my-2 text-xl font-bold"),
                    HeadingLevel::H6 => el.add_attribute("class", "my-2 text-lg font-bold"),
                }
                el
            },
            Tag::BlockQuote(_) => {
                let mut el = VTag::new("blockquote");
                el.add_attribute("class", "p-4 my-4 border-s-4 border-gray-300 bg-gray-50");
                el
            }
            Tag::CodeBlock(code_block_kind) => {
                self.spine.push(VTag::new("pre"));

                let mut el = VTag::new("code");
                if let CodeBlockKind::Fenced(lang) = code_block_kind {
                    // Different color schemes may be used for different code blocks,
                    // but a different library (likely js based at the moment) would be necessary to
                    // actually provide the highlighting support by locating the
                    // language classes and applying dom transforms on their contents.
                    match lang.as_ref() {
                        "html" => el.add_attribute("class", "html-language"),
                        "rust" => el.add_attribute("class", "rust-language"),
                        "java" => el.add_attribute("class", "java-language"),
                        "c" => el.add_attribute("class", "c-language"),
                        _ => {} // Add your own language highlighting support
                    };
                }

                el
            }
            Tag::List(None) => {
                let mut el = VTag::new("ul");
                el.add_attribute("class", "list-inside list-disc");
                el
            }
            Tag::List(Some(1)) => {
                let mut el = VTag::new("ol");
                el.add_attribute("class", "list-inside list-decimal");
                el
            }
            Tag::List(Some(ref start)) => {
                let mut el = VTag::new("ol");
                el.add_attribute("class", "list-inside list-decimal");
                el.add_attribute("start", start.to_string());
                el
            }
            Tag::Item => VTag::new("li"),
            Tag::Table(alignment) => {
                self.open_table_ctx(alignment);
                let mut el = VTag::new("table");
                el.add_attribute("class", "w-full my-4 text-sm text-left text-gray-500");
                el
            }
            Tag::TableHead => {
                let ctx = self.get_table_ctx();
                ctx.next_cell_index = 0;
                ctx.in_head = true;
                let mut th = VTag::new("thead");
                th.add_attribute("class", "text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-200 dark:text-gray-400");
                self.spine.push(th);
                VTag::new("tr")
            }
            Tag::TableRow => {
                let ctx = self.get_table_ctx();
                ctx.next_cell_index = 0;
                if !ctx.has_body {
                    ctx.has_body = true;
                    self.spine.push(VTag::new("tbody"));
                }
                let mut tr = VTag::new("tr");
                tr.add_attribute("class", "bg-white border-b dark:bg-gray-300 dark:border-gray-200");
                tr
            }
            Tag::TableCell => {
                let ctx = self.get_table_ctx();
                let idx = ctx.next_cell_index;
                ctx.next_cell_index += 1;

                let mut tag = if ctx.in_head {
                    let mut th = VTag::new("th");
                    th.add_attribute("class", "px-6 py-3");
                    th.add_attribute("scope", "col");
                    th
                } else {
                    let mut td = VTag::new("td");
                    td.add_attribute("class", "px-6 py-4");
                    td
                };
                match &ctx.alignment[idx] {
                    Alignment::None => {}
                    Alignment::Left => {
                        tag.add_attribute("class", "text-left");
                    }
                    Alignment::Center => {
                        tag.add_attribute("class", "text-center");
                    }
                    Alignment::Right => {
                        tag.add_attribute("class", "text-right");
                    }
                }
                tag
            }
            Tag::Emphasis => {
                let mut el = VTag::new("span");
                el.add_attribute("class", "italic");
                el
            }
            Tag::Strong => {
                let mut el = VTag::new("span");
                el.add_attribute("class", "font-bold");
                el
            }
            Tag::Link {
                ref dest_url,
                ref title,
                link_type: _,
                id: _,
            } => {
                let mut el = VTag::new("span");
                el.add_attribute("href", dest_url.to_string());
                el.add_attribute("class", "font-medium text-blue-600 dark:text-blue-500 hover:underline");
                let title = title.clone().into_string();
                if !title.is_empty() {
                    el.add_attribute("title", title);
                }
                el
            }
            Tag::Image {
                ref dest_url,
                ref title,
                link_type: _,
                id: _,
            } => {
                let mut el = VTag::new("img");
                el.add_attribute("src", dest_url.to_string());
                el.add_attribute("class", "h-auto my-4 max-w-full rounded-lg");
                let title = title.clone().into_string();
                if !title.is_empty() {
                    el.add_attribute("title", title);
                }
                el
            }
            Tag::Strikethrough => {
                let mut el = VTag::new("span");
                el.add_attribute("class", "line-through");
                el
            }
            // Footnotes are not rendered as anything special
            Tag::FootnoteDefinition(ref _footnote_id) => VTag::new("span"),
            Tag::HtmlBlock => VTag::new("div"),
            _ => {
                gloo_console::log!(format!("Unhandled tag: {tag:#?}"));
                VTag::new("div")
            }
        };
        self.spine.push(wrapper);
    }

    fn end_tag(&mut self, tag: TagEnd) {
        self.pop_spine();
        match tag {
            TagEnd::CodeBlock => {
                self.pop_spine(); // Close <pre>
            }
            TagEnd::TableHead => {
                self.pop_spine(); // Close <thead>
                self.get_table_ctx().in_head = false;
            }
            TagEnd::Table => {
                let ctx = self.close_table_ctx();
                if ctx.has_body {
                    self.pop_spine(); // Close <tbody>
                }
            }
            _ => {}
        }
    }

    fn write_event(&mut self, ev: Event) {
        match ev {
            Event::Start(tag) => self.start_tag(tag),
            Event::End(tag) => self.end_tag(tag),
            Event::Text(text) => self.add_child(VText::new(text.to_string()).into()),
            Event::Rule => {
                let mut hr = VTag::new("hr");
                hr.add_attribute("class", "w-48 h-1 mx-auto my-4 bg-gray-100 border-0 rounded md:my-10 dark:bg-gray-200");
                self.add_child(hr.into())
            }
            Event::SoftBreak => self.add_child(VText::new("\n").into()),
            Event::HardBreak => self.add_child(VTag::new("br").into()),
            _ => {},
        };
    }
}

/// Renders a string of Markdown to HTML with the default options
/// (footnotes disabled, tables disabled).
pub fn render_markdown(src: &str) -> Html {
    let mut writer = TagWriter::new();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    for ev in Parser::new_ext(src, options) {
        writer.write_event(ev);
    }

    writer.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_markdown() {
        let markdown_input = concat!(
            "# My Heading\n",
            "## My Heading\n",
            "### My Heading\n",
            "\n",
            "My paragraph.\n",
            "\n",
            "* a\n",
            "* b\n",
            "* c\n",
            "\n",
            "1. d\n",
            "2. e\n",
            "3. f\n",
            "\n",
            "> my block quote\n",
            "\n",
            "```\n",
            "my code block\n",
            "```\n",
            "\n",
            "*emphasis*\n",
            "**strong**\n",
            "~~strikethrough~~\n",
            "[My Link](http://example.com)\n",
            "![My Image](https://flowbite.com/docs/images/examples/image-1@2x.jpg)\n",
            "\n",
            "| a | b |\n",
            "| - | - |\n",
            "| c | d |\n",
            "| c | d |\n",
            "| c | d |\n",
            "| c | d |\n",
            "\n",
            "hello[^1]\n",
            "[^1]: my footnote\n",
        );

        let _ = render_markdown(&markdown_input);
        // TODO: Add assertions
    }
}