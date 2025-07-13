use clap::{Arg, Command};
use regex::Regex;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// アプリケーションの設定を保持する構造体
#[derive(Debug)]
struct Config {
    /// 探索の開始パス
    start_path: PathBuf,
    /// 最大階層数（None の場合は無制限）
    max_depth: Option<usize>,
    /// ディレクトリのみ表示するかどうか
    directories_only: bool,
    /// 除外するパターンのリスト
    exclude_patterns: Vec<Regex>,
}

impl Config {
    /// コマンドライン引数から設定を構築する
    ///
    /// # Returns
    /// 構築された設定
    fn from_args() -> Self {
        let matches = Command::new("rs_tree")
            .version("0.1.0")
            .author("Your Name <your.email@example.com>")
            .about("A cross-platform tree command implementation in Rust")
            .arg(
                Arg::new("directory")
                    .help("Directory to list")
                    .value_name("DIRECTORY")
                    .default_value(".")
                    .index(1),
            )
            .arg(
                Arg::new("max-depth")
                    .short('L')
                    .long("max-depth")
                    .value_name("LEVEL")
                    .help("Descend only level directories deep")
                    .value_parser(clap::value_parser!(usize)),
            )
            .arg(
                Arg::new("directories-only")
                    .short('d')
                    .long("directories-only")
                    .help("List directories only")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("exclude")
                    .short('I')
                    .long("exclude")
                    .value_name("PATTERN")
                    .help("Exclude files/directories matching pattern")
                    .action(clap::ArgAction::Append),
            )
            .get_matches();

        let start_path = PathBuf::from(matches.get_one::<String>("directory").unwrap());
        let max_depth = matches.get_one::<usize>("max-depth").copied();
        let directories_only = matches.get_flag("directories-only");

        let exclude_patterns = if let Some(patterns) = matches.get_many::<String>("exclude") {
            patterns
                .filter_map(|pattern| {
                    match Regex::new(pattern) {
                        Ok(regex) => Some(regex),
                        Err(e) => {
                            eprintln!("Invalid regex pattern '{}': {}", pattern, e);
                            None
                        }
                    }
                })
                .collect()
        } else {
            Vec::new()
        };

        Config {
            start_path,
            max_depth,
            directories_only,
            exclude_patterns,
        }
    }
}

/// ツリー構造のノードを表現する構造体
#[derive(Debug)]
struct TreeNode {
    /// ファイル名
    name: String,
    /// ディレクトリかどうか
    is_dir: bool,
    /// 子ノードのリスト
    children: Vec<TreeNode>,
}

impl TreeNode {
    /// 新しいTreeNodeを作成する
    ///
    /// # Arguments
    /// * `path` - ノードのパス
    /// * `name` - ファイル名
    /// * `is_dir` - ディレクトリかどうか
    ///
    /// # Returns
    /// 新しいTreeNodeインスタンス
    fn new(name: String, is_dir: bool) -> Self {
        TreeNode {
            name,
            is_dir,
            children: Vec::new(),
        }
    }

    /// 子ノードを追加する
    ///
    /// # Arguments
    /// * `child` - 追加する子ノード
    fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

    /// ツリー構造を表示する
    ///
    /// # Arguments
    /// * `prefix` - 表示用のプレフィックス
    /// * `is_last` - 最後の子ノードかどうか
    /// * `file_count` - ファイル数のカウンタ
    /// * `dir_count` - ディレクトリ数のカウンタ
    /// * `show_files` - ファイルを表示するかどうか
    fn display(&self, prefix: &str, is_last: bool, file_count: &mut usize, dir_count: &mut usize, show_files: bool) {
        // ルートディレクトリ以外を表示
        if !prefix.is_empty() {
            let connector = if is_last { "└── " } else { "├── " };
            println!("{}{}{}", prefix, connector, self.name);
        }

        // 統計情報の更新（ルート以外）
        if !prefix.is_empty() {
            if self.is_dir {
                *dir_count += 1;
            } else if show_files {
                *file_count += 1;
            }
        }

        // 子ノードの表示
        let child_prefix = if prefix.is_empty() {
            String::new()
        } else {
            format!("{}{}", prefix, if is_last { "    " } else { "│   " })
        };

        for (i, child) in self.children.iter().enumerate() {
            let is_child_last = i == self.children.len() - 1;
            child.display(&child_prefix, is_child_last, file_count, dir_count, show_files);
        }
    }
}

/// ツリー構造の表示を行うメイン構造体
struct TreePrinter {
    config: Config,
    /// 除外されたパスのセット
    excluded_paths: HashSet<PathBuf>,
}

impl TreePrinter {
    /// 新しいTreePrinterを作成する
    ///
    /// # Arguments
    /// * `config` - アプリケーションの設定
    ///
    /// # Returns
    /// 新しいTreePrinterインスタンス
    fn new(config: Config) -> Self {
        TreePrinter {
            config,
            excluded_paths: HashSet::new(),
        }
    }

    /// ファイル名が除外対象かどうかを判定する
    ///
    /// # Arguments
    /// * `file_name` - 判定対象のファイル名
    ///
    /// # Returns
    /// 除外対象の場合true
    fn should_exclude(&self, file_name: &str) -> bool {
        for pattern in &self.config.exclude_patterns {
            if pattern.is_match(file_name) {
                return true;
            }
        }
        false
    }

    /// パスが除外されたパスの子要素かどうかを判定する
    ///
    /// # Arguments
    /// * `path` - 判定対象のパス
    ///
    /// # Returns
    /// 除外されたパスの子要素の場合true
    fn is_descendant_of_excluded(&self, path: &Path) -> bool {
        for excluded in &self.excluded_paths {
            if path.starts_with(excluded) && path != excluded {
                return true;
            }
        }
        false
    }

    /// ディレクトリ構造を再帰的に構築する
    ///
    /// # Arguments
    /// * `dir_path` - 探索するディレクトリのパス
    /// * `current_depth` - 現在の深度
    ///
    /// # Returns
    /// 構築されたTreeNodeのオプション
    fn build_tree_recursive(&mut self, dir_path: &Path, current_depth: usize) -> Option<TreeNode> {
        // 最大深度チェック
        if let Some(max_depth) = self.config.max_depth {
            if current_depth > max_depth {
                return None;
            }
        }

        // 除外されたパスの子要素かチェック
        if self.is_descendant_of_excluded(dir_path) {
            return None;
        }

        let name = if dir_path == self.config.start_path {
            dir_path.display().to_string()
        } else {
            dir_path.file_name()?.to_string_lossy().to_string()
        };

        let mut node = TreeNode::new(name, true);

        // ディレクトリの内容を読み取り
        let mut entries = match std::fs::read_dir(dir_path) {
            Ok(entries) => {
                let collected: Result<Vec<_>, _> = entries.collect();
                match collected {
                    Ok(entries) => entries,
                    Err(_) => return Some(node),
                }
            }
            Err(_) => return Some(node),
        };

        // ファイル名でソート
        entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

        for entry in entries {
            let path = entry.path();
            let is_dir = path.is_dir();
            let file_name = entry.file_name().to_string_lossy().to_string();

            // 除外パターンのチェック
            if self.should_exclude(&file_name) {
                // 除外されたパスを記録
                self.excluded_paths.insert(path.clone());
                continue;
            }

            // ディレクトリ専用モードでファイルをスキップ
            if self.config.directories_only && !is_dir {
                continue;
            }

            if is_dir {
                // 再帰的にサブディレクトリを処理
                if let Some(child_node) = self.build_tree_recursive(&path, current_depth + 1) {
                    node.add_child(child_node);
                }
            } else {
                // ファイルノードを追加
                let file_node = TreeNode::new(file_name, false);
                node.add_child(file_node);
            }
        }

        Some(node)
    }

    /// ツリー構造を表示する
    fn display_tree(&mut self) {
        let start_path = self.config.start_path.clone();
        let show_files = !self.config.directories_only;
        
        if let Some(root) = self.build_tree_recursive(&start_path, 0) {
            let mut file_count = 0;
            let mut dir_count = 0;

            // ルートディレクトリ名を表示
            println!("{}", root.name);

            // 子ノードを表示（ルートの子供は常に表示）
            for (i, child) in root.children.iter().enumerate() {
                let is_last = i == root.children.len() - 1;
                let connector = if is_last { "└── " } else { "├── " };
                println!("{}{}", connector, child.name);
                
                // 統計情報の更新
                if child.is_dir {
                    dir_count += 1;
                } else if show_files {
                    file_count += 1;
                }
                
                // 子ノードの子を表示
                let child_prefix = if is_last { "    " } else { "│   " };
                for (j, grandchild) in child.children.iter().enumerate() {
                    let is_grandchild_last = j == child.children.len() - 1;
                    grandchild.display(&child_prefix, is_grandchild_last, &mut file_count, &mut dir_count, show_files);
                }
            }

            // 統計情報の表示
            self.display_statistics(dir_count, file_count);
        }
    }

    /// 統計情報を表示する
    ///
    /// # Arguments
    /// * `dir_count` - ディレクトリ数
    /// * `file_count` - ファイル数
    fn display_statistics(&self, dir_count: usize, file_count: usize) {
        println!();
        if self.config.directories_only {
            println!("{} directories", dir_count);
        } else {
            println!("{} directories, {} files", dir_count, file_count);
        }
    }
}

/// メイン関数
fn main() {
    let config = Config::from_args();
    
    // 開始パスの存在確認
    if !config.start_path.exists() {
        eprintln!("Error: Directory '{}' does not exist", config.start_path.display());
        std::process::exit(1);
    }

    if !config.start_path.is_dir() {
        eprintln!("Error: '{}' is not a directory", config.start_path.display());
        std::process::exit(1);
    }

    let mut tree_printer = TreePrinter::new(config);
    tree_printer.display_tree();
}