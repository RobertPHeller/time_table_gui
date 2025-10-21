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
//  Last Modified : <251021.1307>
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
use crate::buttonwidget::*;
use crate::buttonbox::*;
use crate::stdmenubar::*;
use std::collections::HashMap;
//use proc_macro2::TokenStream;
use tk::opt::{TkOption,TtkButtonOpt,TtkCheckbuttonOpt,TtkRadiobuttonOpt,
                TtkMenubuttonOpt,OptPair};
use tuplex::IntoHomoTuple;

pub struct MainWindow<Inst: std::marker::Copy + 'static> {
    // Hull - MainFrame
    hull: MainFrame<Inst>,
    // Components
    scrollwindow: ScrollWindow<Inst>,
    wipmessage: TkMessage<Inst>,
    main: TtkFrame<Inst>,
    buttons: ButtonBox<Inst>,
    panewindow: TtkPanedwindow<Inst>,
    // variables (fields)
    slideouts: HashMap<String,TtkFrame<Inst>>,
    slideout_pathnames: Vec<&'static str>,
    toolbars: HashMap<String,(TtkFrame<Inst>,usize)>,
    toolbuttonmap: HashMap<&'static str,ButtonWidget<Inst>>,
    numtoolbars: usize,
    progress: f64,
    status: String,
    
}

pub enum ButtonType {
    //   -text   -image -command 
    Plain(String,String,fn()),
    //   -text   -image -command -offvalue -onvalue 
    Check(String,String,fn(),     i32,        i32),
    // -text   -image -command -value
    Radio(String,String,fn(), i32),
    // -text   -image -menu
    Menu(String,String,String),
}

impl<Inst: std::marker::Copy + 'static> Setwidget<TkCanvas<Inst>> for MainWindow<Inst> {
    fn setwidget(&mut self, wid: TkCanvas<Inst>) -> TkResult<()>
    {
        self.scrollwindow.setwidget(wid)
    }
}

impl<Inst: std::marker::Copy + 'static> Setwidget<TkText<Inst>> for MainWindow<Inst> {
    fn setwidget(&mut self, wid: TkText<Inst>) -> TkResult<()>
    {
        self.scrollwindow.setwidget(wid)
    }
}

impl<Inst: std::marker::Copy + 'static> Setwidget<TkListbox<Inst>> for MainWindow<Inst> {
    fn setwidget(&mut self, wid: TkListbox<Inst>) -> TkResult<()>
    {
        self.scrollwindow.setwidget(wid)
    }
}

impl<Inst: std::marker::Copy + 'static> Setwidget<TtkTreeview<Inst>> for MainWindow<Inst> {
    fn setwidget(&mut self, wid: TtkTreeview<Inst>) -> TkResult<()>
    {
        self.scrollwindow.setwidget(wid)
    }
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
        let scrollwindow = ScrollWindow::new(&main,"scrollwindow",
                                        ScrollOpt::Both,ScrollOpt::Both,
                                        SidesOpt::Se,0,1,
                                        true)?;
        scrollwindow.pack(-fill("both") -expand("yes"))?;
        let wipmessage = main.add_message("wipmessage" -aspect(1500) -anchor("w") -justify("left"))?;
        wipmessage.pack(-fill("x") -anchor("w"))?;
        let buttons = ButtonBox::new(&panewindow,"buttons",
                            BBOrient::Vertical,BAlignment::Center)?;
        panewindow.add(&buttons,-weight(0))?;
        let this = Self {hull: hull, scrollwindow: scrollwindow, 
                         wipmessage: wipmessage, main: main,
                         buttons: buttons, panewindow: panewindow,
                         slideouts: HashMap::new(),
                         slideout_pathnames: Vec::new(),
                         toolbars: HashMap::new(),
                         toolbuttonmap: HashMap::new(),
                         numtoolbars: 0, progress: 0.0, status: String::new() };
        Ok(this)                    
    }
    pub fn getframe(&self) -> &ScrollWindow<Inst> {&self.scrollwindow}
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
    pub fn buttons_add_button<Opts>(&mut self,name: String, options: impl Into<PathOptsWidgets<Opts,()>>) -> TkResult<TtkButton<Inst>>
    where Opts: IntoHomoTuple<TtkButtonOpt>
            + IntoHomoTuple<OptPair>
    {
        self.buttons.add_button(name,options)
    }
    pub fn buttons_add_checkbutton<Opts>(&mut self,name: String, options: impl Into<PathOptsWidgets<Opts,()>>) -> TkResult<TtkCheckbutton<Inst>>
    where Opts: IntoHomoTuple<TtkCheckbuttonOpt>
            + IntoHomoTuple<OptPair>
    {
        self.buttons.add_checkbutton(name,options)
    }
    pub fn buttons_add_radiobutton<Opts>(&mut self,name: String, options: impl Into<PathOptsWidgets<Opts,()>>) -> TkResult<TtkRadiobutton<Inst>>
    where Opts: IntoHomoTuple<TtkRadiobuttonOpt>
            + IntoHomoTuple<OptPair>
    {
        self.buttons.add_radiobutton(name,options)
    }
    pub fn buttons_add_menubutton<Opts>(&mut self,name: String, options: impl Into<PathOptsWidgets<Opts,()>>) -> TkResult<TtkMenubutton<Inst>>
    where Opts: IntoHomoTuple<TtkMenubuttonOpt>
            + IntoHomoTuple<OptPair>
    {
        self.buttons.add_menubutton(name,options)
    }
    pub fn slideout_add (&mut self,name: &str) 
                    -> TkResult<Option<TtkFrame<Inst>>>
    {
        if self.slideouts.contains_key(&name.to_string()) {
            return Ok(None);
        }
        //let mut framepath = name.to_string().clone();
        //framepath.make_ascii_lowercase();
        //let fp: &'static str = &framepath;
        //self.slideout_pathnames.push(fp);
        //let rawpath = *self.slideout_pathnames.last().unwrap();
        //let path = <&str as Into<tk::cmd::PathOptsWidgets<(), ()>>>::into(rawpath);
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
    pub fn toolbar_add (&mut self,name: &str) -> TkResult<()>
    {
        if !self.toolbars.contains_key(&name.to_string()) {
            self.toolbars.insert(name.to_string(),(self.hull.addtoolbar()?,self.numtoolbars));
            self.numtoolbars += 1;
        }
        Ok(())
    }
    pub fn toolbar_show (&self,name: &str) ->TkResult<()>
    {
        match self.toolbars.get(&name.to_string()) {
            None => Ok(()),
            Some((frame,index)) => self.hull.showtoolbar(*index,true),
        }
    }
    pub fn toolbar_hide (&self,name: &str) ->TkResult<()>
    {
        match self.toolbars.get(&name.to_string()) {
            None => Ok(()),
            Some((frame,index)) => self.hull.showtoolbar(*index,false),
        }
    }
    pub fn toolbar_setbuttonstate (&self,name: &str,state_: StateType)  ->TkResult<()>
    {
        match self.toolbars.get(&name.to_string()) {
            None => Ok(()),
            Some((frame,index)) => {
                for b in frame.winfo_children()?.iter() {
                    let path = b.path();
                    match self.toolbuttonmap.get(path) {
                        None => (),
                        Some(b) => {
                            match b {
                                ButtonWidget::Plain(w) => {
                                    w.configure(-state(match state_ {
                                        StateType::Disabled => "disabled",
                                        StateType::Normal   => "normal",
                                    }))?;
                                },
                                ButtonWidget::Check(w) => {
                                    w.configure(-state(match state_ {
                                        StateType::Disabled => "disabled",
                                        StateType::Normal   => "normal",
                                    }))?;
                                },
                                ButtonWidget::Radio(w) => {
                                    w.configure(-state(match state_ {
                                        StateType::Disabled => "disabled",
                                        StateType::Normal   => "normal",
                                    }))?;
                                },
                                ButtonWidget::Menu(w) => {
                                    w.configure(-state(match state_ {
                                        StateType::Disabled => "disabled",
                                        StateType::Normal   => "normal",
                                    }))?;
                                },
                            }
                        },
                    }
                }
                Ok(())
            },
        }
    }
    pub fn toolbar_addbutton(&mut self,name: &str, bname: &'static str, type_: ButtonType) 
            -> TkResult<()>
    {
        match self.toolbars.get(&name.to_string()) {
            None => Ok(()), 
            Some((frame,index)) => {
                match type_ {
                    ButtonType::Plain(text_,image_,command_) => {
                        let w = frame.add_ttk_button(
                            bname
                            -text(text_) 
                            -image(image_) 
                            /*-command()*/
                            -style("Toolbutton")
                        )?;
                        self.toolbuttonmap.insert(w.path(),
                                            ButtonWidget::Plain(w));
                    },
                    ButtonType::Check(text_,image_,command_,off,on) => {
                        let w = frame.add_ttk_checkbutton(
                            bname
                            -text(text_) 
                            -image(image_) 
                            /*-command()*/
                            -offvalue(off)
                            -onvalue(on)
                            /*-style("Toolbutton")*/
                        )?;
                        self.toolbuttonmap.insert(w.path(),
                                            ButtonWidget::Check(w));
                    },
                    ButtonType::Radio(text_,image_,command_,value_) => {
                        let w = frame.add_ttk_radiobutton(
                            bname
                            -text(text_) 
                            -image(image_) 
                            /*-command()*/
                            -value(value_)    
                            -style("Toolbutton")
                        )?;
                        self.toolbuttonmap.insert(w.path(),
                                            ButtonWidget::Radio(w));

                    },
                    ButtonType::Menu(text_,image_,menu_) => {
                        let w = frame.add_ttk_menubutton(
                            bname
                            -text(text_) 
                            -image(image_)
                            -menu(menu_)
                            -style("Toolbutton")
                        )?;
                        self.toolbuttonmap.insert(w.path(),
                                            ButtonWidget::Menu(w));
                    },
                }
                Ok(())
            },
        }
    }
    //[#cfg(false)]
    //pub fn toolbar_buttonconfigure<Opts>(&self,name: &str, bname: &str, options: impl Into<PathOptsWidgets<Opts,()>>) -> TkResult<Option<()>> 
    //where Opts: IntoHomoTuple<TtkButtonOpt>
    //        + IntoHomoTuple<OptPair>
    //{
    //    match self.toolbars.get(&name.to_string()) {
    //        None => Ok(None), 
    //        Some((frame,index)) => {
    //            let toolbarpath = String::from(frame.path());
    //            let buttonpath: &str = &(toolbarpath+"."+bname);
    //            match self.toolbuttonmap.get(buttonpath) {
    //                None => Ok(None),
    //                Some(b) => {
    //                    match b {
    //                        ButtonWidget::Plain(w) => {
    //                            w.configure(options)?;
    //                        },
    //                        ButtonWidget::Check(w) => {
    //                            w.configure(options)?;
    //                        },
    //                        ButtonWidget::Radio(w) => {
    //                            w.configure(options)?;
    //                        },
    //                        ButtonWidget::Menu(w) => {
    //                            w.configure(options)?;
    //                        },
    //                    };
    //                    Ok(Some(()))
    //                },
    //            }
    //        },
    //    }
    //}
    // ToDo: toolbar_buttoncget
    //       setstatus
    //       setprogress
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
        let mut main = MainWindow::new(&root,"main",600,400,SeparatorType::Both,
                                    &MenuType::new_std_menu())?;
        let frame = main.getframe();
        eprintln!("frame's path is {}",frame.path());
        let canvas = frame.add_canvas("c" -width(600) -height(400))?;
        main.setwidget(canvas)?;
        canvas.configure( -scrollregion( (-600, -400, 600, 400) ))?;
        canvas.create_oval( -10.0, -10.0, 10.0, 10.0, -fill("red"))?; 
        main.buttons_add_button(String::from("test"),"test" -text("test") -command( tclosure!( tk,  || -> TkResult<()> {Ok(eprintln!("Test Button"))})))?;
        main.buttons_show()?;
        main.pack(-fill("both"))?;
        main.menu_entryconfigure("file",0,-command( tclosure!( tk,  || -> TkResult<()> {Ok(eprintln!("New"))})))?;
        main.menu_entryconfigure("file",4,-command(("destroy", ".")))?;
        main.menu_entryconfigure("file",5,-command("exit"))?;
        Ok( main_loop() )
    }
}
