// 一个简化版的nano编辑器实现，使用Rust风格和pancurses库

use pancurses::{initscr, endwin, Input, Window};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::vec::Vec;

// 编辑器配置
struct EditorConfig {
    line_numbers: bool,
    autoindent: bool,
    tabsize: usize,
}

// 编辑器状态
struct Editor {
    config: EditorConfig,
    buffer: Vec<String>,
    filename: Option<String>,
    cursor_x: usize,
    cursor_y: usize,
    screen_start_y: usize,
    modified: bool,
    screen_rows: usize,  // 屏幕行数
}

impl Editor {
    // 创建新的编辑器实例
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
            screen_rows: 24,  // 默认值
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

    // 渲染编辑器
    fn render(&mut self, window: &Window, rows: i32, cols: i32) {
        window.clear();
        
        // 更新屏幕行数
        self.screen_rows = rows as usize;
        
        let rows_usize = rows as usize;
        let cols_usize = cols as usize;
        
        // 显示文件内容
        for i in 0..rows_usize - 2 { // 留出2行用于状态栏
            let line_index = self.screen_start_y + i;
            if line_index >= self.buffer.len() {
                break;
            }
            
            let line = &self.buffer[line_index];
            let mut display_line = line.clone();
            
            // 如果启用了行号
            if self.config.line_numbers {
                let line_num = format!("{:4} ", line_index + 1);
                window.mvprintw(i as i32, 0, &line_num);
                window.mvprintw(i as i32, 5, &display_line);
            } else {
                window.mvprintw(i as i32, 0, &display_line);
            }
        }
        
        // 显示状态栏
        let status_line = format!(
            "{}{} 行: {}, 列: {} 按 Ctrl+Q 退出",
            self.filename.as_ref().map_or("未命名", |f| f),
            if self.modified { " *" } else { "" },
            self.cursor_y + 1, 
            self.cursor_x + 1
        );
        window.mvprintw((rows - 2) as i32, 0, &status_line);
        
        // 显示命令提示
        let help_line = "Ctrl+S 保存 | Ctrl+X 剪切 | Ctrl+V 粘贴 | Ctrl+F 查找";
        window.mvprintw((rows - 1) as i32, 0, &help_line);
        
        // 移动光标到正确位置
        let display_y = (self.cursor_y - self.screen_start_y) as i32;
        let display_x = if self.config.line_numbers {
            self.cursor_x as i32 + 5
        } else {
            self.cursor_x as i32
        };
        window.mv(display_y, display_x);
        
        window.refresh();
    }
}

fn main() {
    // 解析命令行参数
    let args: Vec<String> = env::args().collect();
    
    // 初始化屏幕
    let window = initscr();
    window.keypad(true);
    window.nodelay(false);
    
    // 关闭回显，设置为raw模式
    pancurses::noecho();
    pancurses::cbreak();
    
    // 获取屏幕尺寸
    let mut rows = 0;
    let mut cols = 0;
    let (rows_new, cols_new) = window.get_max_yx();
    rows = rows_new;
    cols = cols_new;
    
    // 创建编辑器实例
    let mut editor = Editor::new();
    
    // 如果提供了文件名参数，尝试加载文件
    if args.len() > 1 {
        let path = Path::new(&args[1]);
        if let Err(err) = editor.load_file(path) {
            window.mvprintw(0, 0, &format!("无法打开文件: {}", err));
            window.refresh();
            window.getch();
        }
    }
    
    // 主循环
    loop {
        // 渲染编辑器
        editor.render(&window, rows, cols);
        
        // 获取用户输入
        match window.getch() {
            Some(Input::KeyResize) => {
                    let (rows_new, cols_new) = window.get_max_yx();
                    rows = rows_new;
                    cols = cols_new;
                },
            Some(Input::Character(c)) => {
                match c {
                    // 退出编辑器
                    '\x11' => { // Ctrl+Q
                        if editor.modified {
                            window.mvprintw((rows - 2) as i32, 0, "文件已修改。确定要退出吗? (y/n)");
                            window.refresh();
                            if let Some(Input::Character('y')) = window.getch() {
                                break;
                            }
                        } else {
                            break;
                        }
                    },
                    // 保存文件
                    '\x13' => { // Ctrl+S
                        if let Err(err) = editor.save_file() {
                            window.mvprintw((rows - 2) as i32, 0, &format!("保存失败: {}", err));
                            window.refresh();
                            window.getch();
                        }
                    },
                    // 标准字符输入
                    _ => {
                        editor.insert_char(c);
                    }
                }
            },
            Some(Input::KeyEnter) => {
                editor.insert_newline();
            },
            Some(Input::KeyBackspace) | Some(Input::KeyDC) => {
                editor.delete_char();
            },
            Some(Input::KeyUp) => {
                editor.move_cursor("up");
            },
            Some(Input::KeyDown) => {
                editor.move_cursor("down");
            },
            Some(Input::KeyLeft) => {
                editor.move_cursor("left");
            },
            Some(Input::KeyRight) => {
                editor.move_cursor("right");
            },
            Some(Input::KeyHome) => {
                editor.move_cursor("home");
            },
            Some(Input::KeyEnd) => {
                editor.move_cursor("end");
            },
            _ => {}
        }
        
        // 确保屏幕尺寸更新
        let (rows_new, cols_new) = window.get_max_yx();
        rows = rows_new;
        cols = cols_new;
    }
    
    // 清理屏幕
    endwin();
}