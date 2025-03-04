// Copyright 2016 Avraham Weinstock
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clipboard_win::{Clipboard, get_clipboard_string, set_clipboard_string};

use crate::common::{ClipboardProvider, Result};

pub struct WindowsClipboardContext;

impl WindowsClipboardContext {
    pub fn new() -> Result<Self> {
        Ok(WindowsClipboardContext)
    }
}

impl ClipboardProvider for WindowsClipboardContext {
    fn get_contents(&mut self) -> Result<String> {
        let clipstring = get_clipboard_string();
        match clipstring {
            Ok(_) => Ok(clipstring?),
            Err(_) => Ok("".to_owned())
        }
    }

    fn set_contents(&mut self, data: String) -> Result<()> {
        if data.len() > 0 {
            Ok(set_clipboard_string(&data)?)
        } else {
            _ = Clipboard::new()?.empty()?;
            Ok(())
        }
    }
}
