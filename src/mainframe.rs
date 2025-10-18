// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-10-17 23:23:00
//  Last Modified : <251018.1412>
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
use tk::*;
use tk::cmd::*;
use tcl::*;

use std::ops::Deref;
use std::collections::HashMap;
use crate::stdmenubar::*;
//use crate::scrollwindow::*;
//use crate::buttonbox::*;

pub struct MainFrame<Inst: std::marker::Copy + 'static> {
    hull: TtkFrame<Inst>,
    // component widgets
    userframe: TtkFrame<Inst>,
    topframe: TtkFrame<Inst>,
    botframe: TtkFrame<Inst>,
    status: TtkFrame<Inst>,
    label: TtkLabel<Inst>,
    indframe: TtkFrame<Inst>,
    prgframe: TtkFrame<Inst>, 
    progress: TtkProgressbar<Inst>,
    // variables
    top: TkToplevel<Inst>,
    ntoolbar: u32,
    toolbars: Vec<TtkFrame<Inst>>,
    nindic: u32,
    indicators: Vec<TkLabel<Inst>>,
    menuidmap: HashMap<String, TkMenu<Inst>>,
    tags: HashMap<String, Vec<(TkMenu<Inst>,usize)>>,
    tagstate: HashMap<String, bool>,
    menutags: HashMap<String, String>,
    
}

impl<Inst: std::marker::Copy + 'static> Deref for MainFrame<Inst> {
    type Target = TtkFrame<Inst>;

    fn deref(&self) -> &Self::Target {
        &self.hull
    }
}

pub enum SeparatorType {None, Top, Bottom, Both,}

impl<Inst: std::marker::Copy> MainFrame<Inst> {
    pub fn new(parent: &Widget<Inst>, path: &'static str, width_: u32, height_: u32,
                separator: SeparatorType,menu: &MenuType) -> TkResult<Self> {
        
        let win = parent.add_ttk_frame(path -width(width_) -height(height_))?;
        let top = win.winfo_toplevel()?;
        let userframe = win.add_ttk_frame("frame")?;
        let topframe = win.add_ttk_frame("topf")?;
        let botframe = win.add_ttk_frame("botf")?;
        topframe.pack( -fill("x") )?;
        topframe.grid_columnconfigure(0, -weight(1))?;
        match separator {
            SeparatorType::Top|SeparatorType::Both => {
                win.add_ttk_separator("sep"  -orient("horizontal") )?
                                                .pack(-fill("x"))?;
            },
            _ => (),
        };
        match separator {
            SeparatorType::Bottom|SeparatorType::Both => {
                botframe.add_ttk_separator("sep"  -orient("horizontal") )?
                                                .pack(-fill("x"))?;
            },
            _ => (),
        };
        let status = botframe.add_ttk_frame("status")?;
        let label  = status.add_ttk_label("label")?;
        let indframe = status.add_ttk_frame("indf")?;
        let prgframe   = indframe.add_ttk_frame("prgf")?;
        label.place(-anchor("w") -x_(0) -rely(0.5))?;
        indframe.place(-anchor("ne") -relx(1) -y_(0) -relheight(1))?;
        prgframe.pack( -side("left") -padx(2))?;
        //status.configure(-height(label.winfo_reqheight()))?;
        let progress = status.add_ttk_progressbar("prg" -orient("horizontal"))?;
        status.pack( -fill("x") -pady(2))?;
        botframe.pack(-side("bottom") -fill("x"))?;
        userframe.pack(-fill("both") -expand("yes"))?;
        botframe.add_ttk_sizegrip("sizegrip")?.pack(-side("right"))?;
        let mut this = Self{hull:win, userframe:userframe, topframe:topframe,
                            botframe:botframe, status:status, label:label,
                            indframe:indframe, prgframe:prgframe, 
                            progress:progress, top:top, ntoolbar: 0,
                            toolbars: Vec::new(),
                            nindic: 0, indicators: Vec::new(),
                            menuidmap: HashMap::new(),
                            tags: HashMap::new(), tagstate: HashMap::new(),
                            menutags: HashMap::new(),};
        this._create_menubar(menu.clone())?;
        Ok(this)
    }
    fn _parse_name(name: &str) -> (String,Option<i32>) {
        match name.match_indices('&').next() {
            None => (name.to_string(),None),
            Some((n,m)) => {
                let label = name[0..n].to_string() + &name[n+1..];
                (label,Some(n as i32))
            },
        }
    }
    fn _create_menubar(&mut self,menuspec: MenuType) -> TkResult<()> {
        let menubar = self.top.add_menu("menubar")?;
        let mut current: MenuType = menuspec;
        let mut count = 0;
        loop {
            if current == MenuType::Nil {break;}
            match current {
                MenuType::Menu(name,tags_,menuid,tear,entries,next) => {
                    let (lab, under) = Self::_parse_name(&name);
                    let menuPath: String = if menuid.len() > 0 {
                                        menuid.clone()
                                    } else {
                                        format!("menu{}",count)
                                    };
                    let newmenu = menubar.add_menu( -tearoff(tear))?;
                    self.menuidmap.insert(menuid.clone(),newmenu);
                    let newmenuitem = 
                        if under.is_none() {
                            menubar.add_cascade( -label(lab) -menu(newmenu.path()))?
                        } else {
                            menubar.add_cascade( -label(lab) -underline(under.unwrap()) -menu(newmenu.path()))?
                    };
                    for tag in tags_.split(' ') {
                        let tags_element = self.tags.entry(tag.to_string()).or_insert(Vec::new());
                        tags_element.push((menubar,count));
                        self.tagstate.entry(tag.to_string()).or_insert(true);
                    }
                    let ents: MenuType = *entries;
                    self._create_entries(newmenu,ents)?;
                    current = *next;
                    count += 1;
                },
                _ => panic!("Should not get here!")
            };
        }
        self.top.configure( -menu(menubar) )?;
        Ok(())
    }
    fn _create_entries(&mut self,menuwidget: TkMenu<Inst>,entries: MenuType) -> TkResult<()> {
        let mut count = 
            match menuwidget.cget( tearoff ) {
                Err(p) => 0,
                Ok(obj) => if obj.as_bool() {1} else {0},
        };
        let mut current: MenuType = entries;
        loop {
            if current == MenuType::Nil {break;}
            match current {
                MenuType::Command(name,tags_,accel,next) => {
                    let (lab, under) = Self::_parse_name(&name);
                    let newmenuitem =
                        if under.is_none() {
                            menuwidget.add_command( -label(lab) )?;
                        } else {
                            menuwidget.add_command( -label(lab) -underline(under.unwrap()) )?;
                    };
                    for tag in tags_.split(' ') {
                        let tags_element = self.tags.entry(tag.to_string()).or_insert(Vec::new());
                        tags_element.push((menuwidget,count));
                        self.tagstate.entry(tag.to_string()).or_insert(true);
                    }
                    current = *next;
                },
                MenuType::Cascade(name,tags_,accel,tear,ents,next) => {
                    let (lab, under) = Self::_parse_name(&name);
                    let newmenu = menuwidget.add_menu( -tearoff(tear))?;
                    let newmenuitem = 
                        if under.is_none() {
                            menuwidget.add_cascade( -label(lab) -menu(newmenu.path()))?;
                        } else {
                            menuwidget.add_cascade( -label(lab) -underline(under.unwrap())  -menu(newmenu.path()))?;
                    };
                    for tag in tags_.split(' ') {
                        let tags_element = self.tags.entry(tag.to_string()).or_insert(Vec::new());
                        tags_element.push((menuwidget,count));
                        self.tagstate.entry(tag.to_string()).or_insert(true);
                    }
                    self._create_entries(newmenu,*ents)?;
                    current = *next;
                },
                MenuType::CheckButton(name,tags_,accel,next) => {
                    let (lab, under) = Self::_parse_name(&name);
                    let newmenuitem =
                        if under.is_none() {
                            menuwidget.add_checkbutton( -label(lab) )?;
                        } else {
                            menuwidget.add_checkbutton( -label(lab) -underline(under.unwrap()) )?;
                    };
                    for tag in tags_.split(' ') {
                        let tags_element = self.tags.entry(tag.to_string()).or_insert(Vec::new());
                        tags_element.push((menuwidget,count));
                        self.tagstate.entry(tag.to_string()).or_insert(true);
                    }
                    current = *next;
                },
                MenuType::RadioButton(name,tags_,accel,next) => {
                    let (lab, under) = Self::_parse_name(&name);
                    let newmenuitem =
                        if under.is_none() {
                            menuwidget.add_radiobutton( -label(lab) )?;
                        } else {
                            menuwidget.add_radiobutton( -label(lab) -underline(under.unwrap()) )?;
                    };
                    for tag in tags_.split(' ') {
                        let tags_element = self.tags.entry(tag.to_string()).or_insert(Vec::new());
                        tags_element.push((menuwidget,count));
                        self.tagstate.entry(tag.to_string()).or_insert(true);
                    }
                    current = *next;
                },
                MenuType::Separator(next) => {
                    menuwidget.add_separator( -columnbreak(1) )?;
                    current = *next;
                },
                _ => panic!("Should not get here"),
            };
            count += 1;
        }
        Ok(())
    }
    fn _parse_accelerator (desc: &str) -> TkResult<(String,String)> {
        Ok((String::new(),String::new()))
    }
    pub fn getframe(&self) -> TtkFrame<Inst> {self.userframe}
    pub fn addtoolbar(&mut self) -> TkResult<TtkFrame<Inst>> {
        let index = self.ntoolbar;
        let toolframe = self.topframe.add_ttk_frame(-padding(1))?
                    .grid(-column(0) -row(index) -sticky("ew"))?;
        let toolbar = toolframe.add_ttk_frame("tb" -padding(2))?
                    .pack(-anchor("w") -expand("yes") -fill("x"))?;
        self.ntoolbar += 1;
        self.toolbars.push(toolbar);
        Ok(toolbar)
    }
    pub fn gettoolbar (&self, index: usize) -> Option<&TtkFrame<Inst>> {
        self.toolbars.get(index)
    }
    pub fn addindicator (&mut self) -> TkResult<usize> {
        let index = self.nindic;
        let indicator = self.indframe.add_label(-bitmap("@grey50") -relief("sunken") -takefocus(false))?;
        indicator.pack(-side("left") -anchor("w") -padx(2) -fill("y") -expand("yes"))?;
        self.indicators.push(indicator);
        Ok(self.indicators.len())
    }
    pub fn getindicator(&self,index: usize) -> Option<&TkLabel<Inst>> {
        self.indicators.get(index)
    }
    pub fn getmenu (&self, menuid_: &str) -> TkResult<Option<&TkMenu<Inst>>> {
        Ok(self.menuidmap.get(&menuid_.to_string()))
    }
    pub fn menu_activate(&self, menuid_: &str, index: usize) -> TkResult<()> {
        match self.menuidmap.get(&menuid_.to_string()) {
            None => Ok(()),
            Some(menu) => {
                menu.activate(index as i32)?;
                Ok(())
            },
        }
    }
    pub fn menu_add(&mut self, menuid_: &str, entry: MenuType) -> TkResult<()> {
        match self.menuidmap.get(&menuid_.to_string()) {
            None => Ok(()),
            Some(menuwidget) => {
                let mut icount: i32 = 0;
                let mut count: usize = 0;
                loop {
                    match menuwidget.index(icount)? {
                        None => {break;},
                        Some(i) => {
                            count = icount as usize;
                            icount += 1;
                        }
                    };
                }                            
                match entry {
                    MenuType::Command(name,tags_,accel,next) => {
                        let (lab, under) = Self::_parse_name(&name);
                        let newmenuitem =
                            if under.is_none() {
                                menuwidget.add_command( -label(lab) )?;
                            } else {
                                menuwidget.add_command( -label(lab) -underline(under.unwrap()) )?;
                        };
                        for tag in tags_.split(' ') {
                            let tags_element = self.tags.entry(tag.to_string()).or_insert(Vec::new());
                            tags_element.push((*menuwidget,count));
                            self.tagstate.entry(tag.to_string()).or_insert(true);
                        }
                    },
                    MenuType::Cascade(name,tags_,accel,tear,ents,next) => {
                        let (lab, under) = Self::_parse_name(&name);
                        let newmenu = menuwidget.add_menu( -tearoff(tear))?;
                        let newmenuitem = 
                            if under.is_none() {
                                menuwidget.add_cascade( -label(lab) -menu(newmenu.path()))?;
                            } else {
                                menuwidget.add_cascade( -label(lab) -underline(under.unwrap())  -menu(newmenu.path()))?;
                        };
                        for tag in tags_.split(' ') {
                            let tags_element = self.tags.entry(tag.to_string()).or_insert(Vec::new());
                            tags_element.push((*menuwidget,count));
                            self.tagstate.entry(tag.to_string()).or_insert(true);
                        }
                        self._create_entries(newmenu,*ents)?;
                    },
                    MenuType::CheckButton(name,tags_,accel,next) => {
                        let (lab, under) = Self::_parse_name(&name);
                        let newmenuitem =
                            if under.is_none() {
                                menuwidget.add_checkbutton( -label(lab) )?;
                            } else {
                                menuwidget.add_checkbutton( -label(lab) -underline(under.unwrap()) )?;
                        };
                        for tag in tags_.split(' ') {
                            let tags_element = self.tags.entry(tag.to_string()).or_insert(Vec::new());
                            tags_element.push((*menuwidget,count));
                            self.tagstate.entry(tag.to_string()).or_insert(true);
                        }
                    },
                    MenuType::RadioButton(name,tags_,accel,next) => {
                        let (lab, under) = Self::_parse_name(&name);
                        let newmenuitem =
                            if under.is_none() {
                                menuwidget.add_radiobutton( -label(lab) )?;
                            } else {
                                menuwidget.add_radiobutton( -label(lab) -underline(under.unwrap()) )?;
                        };
                        for tag in tags_.split(' ') {
                            let tags_element = self.tags.entry(tag.to_string()).or_insert(Vec::new());
                            tags_element.push((*menuwidget,count));
                            self.tagstate.entry(tag.to_string()).or_insert(true);
                        }
                    },
                    MenuType::Separator(next) => {
                        menuwidget.add_separator( -columnbreak(1) )?;
                    },
                    _ => (),
                };
                Ok(())
            },
        }
    }    
    pub fn menu_delete(&mut self,menuid_: &str, index1: i32) -> TkResult<()> {
        match self.menuidmap.get(&menuid_.to_string()) {
            None => Ok(()),
            Some(menuwidget) => {
                menuwidget.delete(index1)?;
                Ok(())
            },
        }
    }
    pub fn menu_entrycget(&mut self,menuid_: &str, index1: i32, option: &str)
                        -> TkResult<Option<Obj>> {
        match self.menuidmap.get(&menuid_.to_string()) {
            None => Ok(None),
            Some(menuwidget) => Ok(Some(menuwidget.entrycget(index1, 
                                    option)?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn MainFrame_new () -> TkResult<()> {
        let tk = make_tk!()?;
        let root = tk.root();
        let temp = MainFrame::new(&root,"main",600,400,SeparatorType::Both,
                                    &MenuType::new_std_menu())?;
        let frame = temp.getframe();
        frame.add_canvas("c" -width(600) -height(400))?.pack(-fill("both"))?;
        temp.pack(-fill("both"))?;
        Ok( main_loop() )
    }
}
