use std::borrow::Cow;

/// リーフとコンポジットを動的にチェックするためのエラー
#[derive(Debug)]
pub struct FileSystemError(String);

// -------------------------------------------------------------------------------------------------
// File

/// リーフ．あらかじめ種類が分かるため列挙体を用いている．
#[derive(Clone)]
pub enum File {
    TextFile(String),
    RustFile(String),
    OtherFile(String),
}

impl File {
    pub fn name(&self) -> &str {
        match self {
            File::TextFile(name) => name,
            File::RustFile(name) => name,
            File::OtherFile(name) => name,
        }
    }
}

// -------------------------------------------------------------------------------------------------
// FileComponent

/// 具象コンボーネント
pub enum FileComponent {
    Folder(String, Vec<Box<FileComponent>>),
    File(File),
}

impl FileComponent {
    pub fn new_folder<S: Into<Cow<'static, str>>>(name: S) -> Self {
        let name: Cow<'static, str> = name.into();
        FileComponent::Folder(name.into_owned(), Vec::new())
    }
    pub fn add(&mut self, child: FileComponent) -> Result<(), FileSystemError> {
        match self {
            FileComponent::Folder(_, folder) => {
                folder.push(Box::new(child));
                Ok(())
            }
            FileComponent::File(_) => Err(FileSystemError(
                "ファイルに対してできない操作です".to_string(),
            )),
        }
    }
    pub fn name(&self) -> &str {
        match self {
            FileComponent::Folder(name, _) => name,
            FileComponent::File(file) => file.name(),
        }
    }

    pub fn child(&self) -> Option<&[Box<FileComponent>]> {
        match self {
            FileComponent::Folder(_, folder) => Some(folder),
            FileComponent::File(_) => None,
        }
    }
    pub fn child_mut(&mut self) -> Option<&mut Vec<Box<FileComponent>>> {
        match self {
            FileComponent::Folder(_, folder) => Some(folder),
            FileComponent::File(_) => None,
        }
    }

    pub fn print_recursive(&self) {
        match self {
            FileComponent::Folder(name, folder) => {
                println!("フォルダー: {}です", name);
                for component in folder.iter() {
                    component.print_recursive();
                }
            }
            FileComponent::File(file) => {
                println!("ファイル: {}です", file.name())
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
// 以下は生成用のヘルパー関数・マクロ

pub fn text_file<S: Into<Cow<'static, str>>>(name: S) -> FileComponent {
    let name: Cow<'static, str> = name.into();
    FileComponent::File(File::TextFile(name.into_owned()))
}

#[macro_export]
macro_rules! folder {
    ($name:expr) => {
        $crate::FileComponent::new_folder($name)
    };
    ($name:expr, $($child:expr),*) => {
        {
            let mut folder_comp = $crate::FileComponent::new_folder($name);
            $(
                folder_comp.add($child).unwrap(); // フォルダーであるのは確定しているため
            )*
            folder_comp
        }
    };
}

// -------------------------------------------------------------------------------------------------
// 以下はコンポジットイテレーター

pub struct IntoIter {
    file_component: Option<FileComponent>,
    folder_stack: Vec<Box<FileComponent>>,
}

impl Iterator for IntoIter {
    type Item = File;
    fn next(&mut self) -> Option<Self::Item> {
        let now_file_component = self.file_component.take()?;

        match now_file_component {
            FileComponent::File(file) => {
                if let Some(popped_file_component) = self.folder_stack.pop() {
                    self.file_component = Some(*popped_file_component);
                }

                Some(file)
            }
            FileComponent::Folder(_, folder) => {
                self.folder_stack.extend(folder.into_iter().rev());

                if let Some(popped_file_component) = self.folder_stack.pop() {
                    self.file_component = Some(*popped_file_component);
                }

                self.next()
            }
        }
    }
}

impl IntoIterator for FileComponent {
    type Item = File;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            file_component: Some(self),
            folder_stack: Vec::new(),
        }
    }
}

pub struct FileIter<'a> {
    file_component: Option<&'a FileComponent>,
    folder_stack: Vec<&'a Box<FileComponent>>,
}

impl<'a> Iterator for FileIter<'a> {
    type Item = &'a File;
    fn next(&mut self) -> Option<Self::Item> {
        let now_file_component = self.file_component.take()?;

        match now_file_component {
            FileComponent::File(file) => {
                if let Some(popped_file_component) = self.folder_stack.pop() {
                    self.file_component = Some(popped_file_component.as_ref());
                }

                Some(file)
            }
            FileComponent::Folder(_, folder) => {
                self.folder_stack.extend(folder.into_iter().rev());

                if let Some(popped_file_component) = self.folder_stack.pop() {
                    self.file_component = Some(popped_file_component.as_ref());
                }

                self.next()
            }
        }
    }
}

impl FileComponent {
    pub fn iter_file(&self) -> FileIter<'_> {
        FileIter {
            file_component: Some(self),
            folder_stack: Vec::new(),
        }
    }
}

pub struct FileComponentIter<'a> {
    file_component: Option<&'a FileComponent>,
    folder_stack: Vec<&'a Box<FileComponent>>,
}

impl<'a> Iterator for FileComponentIter<'a> {
    type Item = &'a FileComponent;
    fn next(&mut self) -> Option<Self::Item> {
        let now_file_component = self.file_component.take()?;

        if let FileComponent::Folder(_, folder) = now_file_component {
            self.folder_stack.extend(folder.into_iter().rev());
        }

        if let Some(popped_file_component) = self.folder_stack.pop() {
            self.file_component = Some(popped_file_component.as_ref());
        }

        Some(now_file_component)
    }
}

impl FileComponent {
    pub fn iter_file_component(&self) -> FileComponentIter<'_> {
        FileComponentIter {
            file_component: Some(self),
            folder_stack: Vec::new(),
        }
    }
}
