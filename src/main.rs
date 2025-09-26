use std::env;
use std::fs::File;
use std::io::{self, Read, Write, stdout, stdin};
use std::path::Path;
use std::vec::Vec;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{cursor, clear, event::Key, input::TermRead};

struct EditorConfig {
    line_numbers: bool,
    autoindent: bool,
    tabsize: usize,
}

struct Editor {
    config: EditorConfig,
    buffer: Vec<String>,
    filename: Option<String>,
    cursor_x: usize,
    cursor_y: usize,
    screen_start_y: usize,
    modified: bool,
    screen_rows: usize,
}

impl Editor {
    fn new() -> Self {
        Editor {
            config: EditorConfig {
                line_numbers: false,
                autoindent: false,
                tabsize: 4,
            },
            buffer: vec![String::new()],
            filename: None,
            cursor_x: 0,
            cursor_y: 0,
            screen_start_y: 0,
            modified: false,
            screen_rows: 24,
        }
    }

    // 加载文件内容到缓冲区
    fn load_file(&mut self, path: &Path) -> Result<(), String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| e.to_string())?;

        self.buffer = content.lines().map(|line| line.to_string()).collect();
        if self.buffer.is_empty() {
            self.buffer.push(String::new());
        }

        self.filename = Some(path.to_string_lossy().to_string());
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.screen_start_y = 0;
        self.modified = false;

        Ok(())
    }

    // 保存缓冲区内容到文件
    fn save_file(&mut self) -> Result<(), String> {
        if let Some(filename) = &self.filename {
            let path = Path::new(filename);
            let mut file = File::create(path).map_err(|e| e.to_string())?;
            
            for line in &self.buffer {
                writeln!(file, "{}", line).map_err(|e| e.to_string())?;
            }
            
            self.modified = false;
            Ok(())
        } else {
            Err("No filename specified".to_string())
        }
    }

    // 在当前位置插入字符
    fn insert_char(&mut self, c: char) {
        let line = &mut self.buffer[self.cursor_y];
        line.insert(self.cursor_x, c);
        self.cursor_x += 1;
        self.modified = true;
    }

    // 在当前位置插入新行
    fn insert_newline(&mut self) {
        let current_line = self.buffer[self.cursor_y].clone();
        let remaining = current_line[self.cursor_x..].to_string();
        self.buffer[self.cursor_y] = current_line[..self.cursor_x].to_string();
        self.buffer.insert(self.cursor_y + 1, remaining);
        self.cursor_y += 1;
        self.cursor_x = 0;
        
        // 处理自动缩进
        if self.config.autoindent && self.cursor_y > 0 {
            let prev_line = &self.buffer[self.cursor_y - 1];
            let indent = prev_line.chars().take_while(|c| c.is_whitespace()).count();
            for _ in 0..indent {
                self.insert_char(' ');
            }
        }
        
        self.modified = true;
    }

    // 删除当前位置的字符
    fn delete_char(&mut self) {
        if self.cursor_x > 0 {
            let line = &mut self.buffer[self.cursor_y];
            line.remove(self.cursor_x - 1);
            self.cursor_x -= 1;
            self.modified = true;
        } else if self.cursor_y > 0 {
            // 如果在行首，则合并到上一行
            let current_line = self.buffer.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.cursor_x = self.buffer[self.cursor_y].len();
            self.buffer[self.cursor_y].push_str(&current_line);
            self.modified = true;
        }
    }

    // 移动光标
    fn move_cursor(&mut self, direction: &str) {
        match direction {
            "up" => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.ensure_cursor_visible();
                }
            }
            "down" => {
                if self.cursor_y < self.buffer.len() - 1 {
                    self.cursor_y += 1;
                    self.ensure_cursor_visible();
                }
            }
            "left" => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.buffer[self.cursor_y].len();
                }
            }
            "right" => {
                if self.cursor_x < self.buffer[self.cursor_y].len() {
                    self.cursor_x += 1;
                } else if self.cursor_y < self.buffer.len() - 1 {
                    self.cursor_y += 1;
                    self.cursor_x = 0;
                    self.ensure_cursor_visible();
                }
            }
            "home" => {
                self.cursor_x = 0;
            }
            "end" => {
                self.cursor_x = self.buffer[self.cursor_y].len();
            }
            _ => {}
        }
    }

    // 确保光标在可视区域内
    fn ensure_cursor_visible(&mut self) {
        if self.cursor_y < self.screen_start_y {
            self.screen_start_y = self.cursor_y;
        } else if self.cursor_y >= self.screen_start_y + self.screen_rows - 2 {
            // 留出2行用于状态栏
            self.screen_start_y = self.cursor_y - self.screen_rows + 3;
        }
    }

    // 用 termion 渲染
    fn render<W: Write>(&mut self, stdout: &mut W) {
        write!(stdout, "{}", clear::All).unwrap();
        for i in 0..self.screen_rows - 2 {
            let line_index = self.screen_start_y + i;
            if line_index >= self.buffer.len() { break; }
            let line = &self.buffer[line_index];
            if self.config.line_numbers {
                write!(stdout, "{}{:4} {}", cursor::Goto(1, (i+1) as u16), line_index+1, line).unwrap();
            } else {
                write!(stdout, "{}{}", cursor::Goto(1, (i+1) as u16), line).unwrap();
            }
        }
        // 状态栏
        let status = format!("{}{} 行:{}, 列:{} 按 Ctrl+Q 退出",
            self.filename.as_ref().map_or("未命名", |f| f),
            if self.modified { " *" } else { "" },
            self.cursor_y + 1, self.cursor_x + 1
        );
        write!(stdout, "{}{}", cursor::Goto(1, (self.screen_rows - 1) as u16), status).unwrap();
        // 光标位置
        let display_y = (self.cursor_y - self.screen_start_y) as u16 + 1;
        let display_x = if self.config.line_numbers { self.cursor_x as u16 + 6 } else { self.cursor_x as u16 + 1 };
        write!(stdout, "{}", cursor::Goto(display_x, display_y)).unwrap();
        stdout.flush().unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // 获取终端尺寸
    let (cols, rows) = termion::terminal_size().unwrap();
    let mut editor = Editor::new();
    editor.screen_rows = rows as usize;

    if args.len() > 1 {
        let path = Path::new(&args[1]);
        let _ = editor.load_file(path);
    }

    editor.render(&mut stdout);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => break,
            Key::Char('\n') => editor.insert_newline(),
            Key::Backspace => editor.delete_char(),
            Key::Up => editor.move_cursor("up"),
            Key::Down => editor.move_cursor("down"),
            Key::Left => editor.move_cursor("left"),
            Key::Right => editor.move_cursor("right"),
            Key::Ctrl('s') => { let _ = editor.save_file(); },
            Key::Char(c) => editor.insert_char(c),
            _ => {},
        }
        editor.render(&mut stdout);
    }
}