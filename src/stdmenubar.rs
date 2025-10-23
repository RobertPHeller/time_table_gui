// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-10-18 02:12:18
//  Last Modified : <251023.1210>
//
//  Description	
//
//  Notes
//
//  History
//	
/////////////////////////////////////////////////////////////////////////////
//    Copyright (C) 2025  Robert Heller D/B/A Deepwoods Software
//			51 Locke Hill Road
//			Wendell, MA 01379-9728
//
//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program; if not, write to the Free Software
//    Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
// 
//
//////////////////////////////////////////////////////////////////////////////
use std::boxed::*;

#[derive(PartialEq, Debug, Clone)]
pub enum MenuType {
    //   name   tags   mid    tear  entries      next
    Menu(String,String,String,bool,Box<MenuType>,Box<MenuType>),
    //      name   tags   accel   next
    Command(String,String,String,Box<MenuType>),
    //      name   tags   accel  tear ents          next
    Cascade(String,String,String,bool,Box<MenuType>,Box<MenuType>),
    //          name   tags   accel   next
    CheckButton(String,String,String,Box<MenuType>),
    //          name   tags   accel  next
    RadioButton(String,String,String,Box<MenuType>),
    //        next
    Separator(Box<MenuType>),
    Nil,
}

impl MenuType {
    pub fn new_menu(label: &str,tag: &str, menuid: &str, tearoff: bool, 
                    entries: MenuType, next: MenuType) -> Self {
        MenuType::Menu(label.to_string(),tag.to_string(),menuid.to_string(),
                tearoff,Box::new(entries), Box::new(next))
    }
    pub fn new_command(label: &str,tag: &str,  
                        accel: &str, next: MenuType) -> Self {
        MenuType::Command(label.to_string(),tag.to_string(),
                            accel.to_string(),Box::new(next))
    }
    pub fn new_cascade(label: &str,tag: &str,
                        accel: &str, tearoff: bool,entries: MenuType, next: MenuType) 
                -> Self {
        MenuType::Cascade(label.to_string(),tag.to_string(),
                        accel.to_string(),tearoff,Box::new(entries), Box::new(next))
    }
    pub fn new_checkbutton(label: &str,tag: &str,accel: &str,
                            next: MenuType) -> Self {
        MenuType::CheckButton(label.to_string(),tag.to_string(),
                            accel.to_string(), Box::new(next))
    }
    pub fn new_radiobutton(label: &str,tag: &str,accel: &str,
                            next: MenuType) -> Self {
        MenuType::RadioButton(label.to_string(),tag.to_string(),
                            accel.to_string(), Box::new(next))
    }
    pub fn new_separator(next: MenuType) -> Self {
        MenuType::Separator(Box::new(next))
    }
    pub fn new_std_menu () -> Self {
        let mut _std_help_menu_cs = 
            Self::new_command("Copying","help:copying","",Self::Nil);
        _std_help_menu_cs =
            Self::new_command("Warranty","help:warranty","",_std_help_menu_cs);
        _std_help_menu_cs = 
            Self::new_command("On &Version","help:version","",_std_help_menu_cs);
        _std_help_menu_cs =
            Self::new_command("&Tutorial...","help:tutorial","",_std_help_menu_cs);
        _std_help_menu_cs =
            Self::new_command("&Index...","help:index","",_std_help_menu_cs);
        _std_help_menu_cs =
            Self::new_command("On &Keys...","help:keys","",_std_help_menu_cs);
        _std_help_menu_cs =
            Self::new_command("On &Help...","help:help","",_std_help_menu_cs);
        let _std_help_menu =
            Self::new_menu("&Help","help","help",false,_std_help_menu_cs,Self::Nil);
        let _std_options_menu =
            Self::new_menu("&Options","options","options",false,Self::Nil,_std_help_menu);
        let _std_view_menu =
            Self::new_menu("&View","view","view",false,Self::Nil,_std_options_menu);
        let mut _std_edit_menu_cs =
            Self::new_command("De-select All","edit:deselectall edit:havesel","",MenuType::Nil);
        _std_edit_menu_cs = 
            Self::new_command("Select All","edit:selectall","",_std_edit_menu_cs);
        _std_edit_menu_cs = 
            Self::new_separator(_std_edit_menu_cs);
        _std_edit_menu_cs = 
            Self::new_command("&Delete","edit:delete edit:havesel","Ctrl d",_std_edit_menu_cs);
        _std_edit_menu_cs = 
            Self::new_command("C&lear","edit:clear edit:havesel","",_std_edit_menu_cs);
        _std_edit_menu_cs = 
            Self::new_command("&Copy","edit:copy edit:havesel","Ctrl c",_std_edit_menu_cs);
        _std_edit_menu_cs = 
            Self::new_command("Cu&t","edit:cut edit:havesel","Ctrl x",_std_edit_menu_cs);
        _std_edit_menu_cs = 
            Self::new_command("&Undo","edit:undo","Ctrl z",_std_edit_menu_cs);
        let _std_edit_menu = 
            Self::new_menu("&Edit","edit","edit",false,_std_edit_menu_cs,_std_view_menu);
        let mut _std_file_menu_cs = 
            Self::new_command("E&xit","file:exit","",MenuType::Nil);
        _std_file_menu_cs = 
            Self::new_command("&Close","file:close","",_std_file_menu_cs);
        _std_file_menu_cs = 
            Self::new_command("Save &As...","file:save","Ctrl a",_std_file_menu_cs);
        _std_file_menu_cs =
            Self::new_command("&Save","file:save","Ctrl s",_std_file_menu_cs);
        _std_file_menu_cs =
            Self::new_command("&Open...","file:open","Ctrl o",_std_file_menu_cs);
        _std_file_menu_cs =
            Self::new_command("&New","file:new","Ctrl n",_std_file_menu_cs);
        let _std_file_menu =
            Self::new_menu("&File","file:menu","file",false,_std_file_menu_cs,
                            _std_edit_menu);
        _std_file_menu
    } 
}


