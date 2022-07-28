/* 
   Copyright (c) 2022 ArSysOp.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at
  
       http:  www.apache.org/licenses/LICENSE-2.0
  
   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
  
   SPDX-License-Identifier: Apache-2.0
  
   Contributors:
     ArSysOp - initial API and implementation
*/

use clang::*;

pub trait Name {
    fn get_name(&self) -> String;
}

impl<T: Name> Name for Option<T> {
    fn get_name(&self) -> String {
        match self {
            Some(t) => t.get_name(),
            None => String::from(""),
        }
    }
}

impl Name for String {
    fn get_name(&self) -> String {
        self.to_string()
    }
}

impl Name for Type<'_> {
    fn get_name(&self) -> String {
        self.get_display_name()
    }
}

impl Name for Usr {
    fn get_name(&self) -> String {
        self.0.to_string()
    }
}

impl Name for source::SourceLocation<'_> {
    fn get_name(&self) -> String {
        let location = self.get_presumed_location();
        [location.0, location.1.to_string(), location.2.to_string()].join(":")
    }
}

pub fn return_empty_if_null(name: Option<String>) -> String {
    match name {
        Some(name) => name,
        None => String::from(""),
    }
}