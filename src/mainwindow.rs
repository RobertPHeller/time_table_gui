// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-10-17 13:42:03
//  Last Modified : <251019.1739>
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

use tcl::*;
use tk::*;
use tk::cmd::*;
use std::ops::Deref;
use crate::mainframe::*;
use crate::scrollwindow::*;
use crate::buttonbox::*;
use crate::stdmenubar::*;
use std::collections::HashMap;

pub struct MainWindow<Inst: std::marker::Copy + 'static> {
    // Hull - MainFrame
    hull: MainFrame<Inst>,
    // Components
    scrollwindow: /*ScrollWindow<Inst>*/ TtkFrame<Inst>,
    wipmessage: TkMessage<Inst>,
    main: TtkFrame<Inst>,
    buttons: /*ButtonBox<Inst>*/ TtkFrame<Inst>,
    panewindow: TtkPanedwindow<Inst>,
    // variables (fields)
    slideouts: HashMap<String,TtkFrame<Inst>>,
    slideout_pathnames: Vec<String>,
    toolbars: HashMap<String,TtkFrame<Inst>>,
    numtoolbars: usize,
    progress: f64,
    status: String,
    
}

impl<Inst: std::marker::Copy> MainWindow<Inst> {
    pub fn new(parent: &Widget<Inst>, path: &'static str, width_: u32, 
                height_: u32,separator: SeparatorType,menu: &MenuType) 
                    -> TkResult<Self>
    {
        let hull = MainFrame::new(parent,path,width_,height_,separator,menu)?;
        hull.showstatusbar(StatusShowType::Progression)?;
        let toplevel = hull.winfo_toplevel()?;
        let frame = hull.getframe();
        let panewindow = hull.add_ttk_panedwindow("panewindow" -orient("horizontal"))?;
        panewindow.pack(-expand("yes") -fill("both"))?;
        let main = panewindow.add_ttk_frame("main")?;
        panewindow.add(&main,-weight(3))?;
        let scrollwindow = /* ScrolledWindow::new(&main,"scrollwindow",
                                        ScrollOpt::Both,ScrollOpt::Both,
                                        scrolling)?; */ 
            main.add_ttk_frame("scrollwindow")?;
        scrollwindow.pack(-fill("both") -expand("yes"))?;
        let wipmessage = main.add_message("wipmessage" -aspect(1500) -anchor("w") -justify("left"))?;
        wipmessage.pack(-fill("x") -anchor("w"))?;
        let buttons = /* ButtonBox::new(&panewindow,"buttons",
                            BBOrient::Vertical)?; */
                    panewindow.add_ttk_frame("buttons")?;
        panewindow.add(&buttons,-weight(0))?;
        let this = Self {hull: hull, scrollwindow: scrollwindow, 
                         wipmessage: wipmessage, main: main,
                         buttons: buttons, panewindow: panewindow,
                         slideouts: HashMap::new(),
                         slideout_pathnames: Vec::new(),
                         toolbars: HashMap::new(),
                         numtoolbars: 0, progress: 0.0, status: String::new() };
        Ok(this)                    
    }
    pub fn getframe(&self) -> TtkFrame<Inst> {self.scrollwindow}
    pub fn buttons_hide(&self) -> TkResult<()> {
        self.panewindow.forget(ttk_panedwindow::TtkPane::Widget(&self.buttons))?;
        Ok(())
    }
    pub fn buttons_show(&self) -> TkResult<()> {
        if match self.panewindow.panes() {
            Err(p) => {return Err(p.into());},
            Ok(panes) => panes.len(),
           } == 1 {
            self.panewindow.add(&self.buttons, -weight(0))?;
        } else {
            self.panewindow.insert(tk::TtkInsertPos::Num(1),&self.buttons, -weight(0))?;
        }
        Ok(())
    }
    pub fn slideout_add (&mut self,name: &str) 
                    -> TkResult<Option<TtkFrame<Inst>>>
    {
        if self.slideouts.contains_key(&name.to_string()) {
            return Ok(None);
        }
        //let mut framepath = name.to_string().clone();
        //framepath.make_ascii_lowercase();
        //self.slideout_pathnames.push(framepath);
        //let rawpath = self.slideout_pathnames.last().unwrap().as_str();
        //let path = <&str as Into<tk::cmd::PathOptsWidgets<(), ()>>>::into(rawpath)
        //let frame = self.panewindow.add_ttk_frame(path)?;
        let frame = self.panewindow.add_ttk_frame(())?;
        self.slideouts.insert(name.to_string(),frame);
        Ok(Some(frame))
    }
    pub fn slideout_show(&self,name: &str) -> TkResult<Option<&TtkFrame<Inst>>> 
    {
        if self.slideouts.contains_key(&name.to_string()) {
            let frame = self.slideouts.get(&name.to_string()).unwrap();
            self.panewindow.add(&frame, -weight(0))?;
            Ok(Some(&frame))
        } else {
            Ok(None)
        }
    }
    pub fn slideout_hide(&self,name: &str) -> TkResult<Option<&TtkFrame<Inst>>> 
    {
        if self.slideouts.contains_key(&name.to_string()) {
            let frame = self.slideouts.get(&name.to_string()).unwrap();
            self.panewindow.forget(ttk_panedwindow::TtkPane::Widget(&frame))?;
            Ok(Some(&frame))
        } else {
            Ok(None)
        }        
    }
    pub fn slideout_getframe(&self,name: &str) -> Option<&TtkFrame<Inst>> 
    {
        self.slideouts.get(&name.to_string())
    }
    pub fn slideout_isshownp(&self,name: &str) -> TkResult<bool>
    {
        match self.slideouts.get(&name.to_string()).copied() {
            None => Ok(false),
            Some(frame) => {
                for p in self.panewindow.panes()?.iter() {
                    if p.path() == frame.path() {
                        return Ok(true);
                    }
                }
                Ok(false)
            },
        }
    }
    pub fn slideout_list (&self) -> Vec<&String>
    {
        self.slideouts.keys().collect()
    }
    pub fn slideout_reqwidth(&self, name: &str) -> TkResult<Option<i32>>
    {
        match self.slideouts.get(&name.to_string()).copied() {
            None => Ok(None),
            Some(frame) => {
                let wid = frame.winfo_reqwidth()?;
                Ok(Some(wid))
            }
        }
    }
}


impl<Inst: std::marker::Copy + 'static> Deref for MainWindow<Inst> {
    type Target = MainFrame<Inst>;

    fn deref(&self) -> &Self::Target {
        &self.hull
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn MainWindow_new () -> TkResult<()> {
        let tk = make_tk!()?;
        let root = tk.root();
        let main = MainWindow::new(&root,"main",600,400,SeparatorType::Both,
                                    &MenuType::new_std_menu())?;
        let frame = main.getframe();
        eprintln!("frame's path is {}",frame.path());
        frame.add_canvas("c" -width(600) -height(400))?.pack(-fill("both"))?;
        main.pack(-fill("both"))?;
        main.menu_entryconfigure("file",0,-command( tclosure!( tk,  || -> TkResult<()> {Ok(eprintln!("New"))})))?;
        main.menu_entryconfigure("file",4,-command(("destroy", ".")))?;
        main.menu_entryconfigure("file",5,-command("exit"))?;
        Ok( main_loop() )
    }
}
