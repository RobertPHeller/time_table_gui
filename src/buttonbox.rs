// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-10-17 23:23:24
//  Last Modified : <251021.1213>
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
use tk::opt::{TtkButtonOpt,OptPair,TtkCheckbuttonOpt,TtkRadiobuttonOpt,TtkMenubuttonOpt};
use std::ops::Deref;
use crate::buttonwidget::*;
use std::collections::HashMap;
use tuplex::IntoHomoTuple;

pub enum BBOrient {Vertical, Horizontal}
pub enum BAlignment {Left, Center, Right}

pub struct ButtonBox<Inst: std::marker::Copy + 'static> {
    // Hull   
    hull: TtkFrame<Inst>,  
    // variables
    buttons: HashMap<String,ButtonWidget<Inst>>,
    orient: BBOrient,
    alignment: BAlignment,
}

impl<Inst: std::marker::Copy + 'static> Deref for ButtonBox<Inst> {
     type Target = TtkFrame<Inst>;

    fn deref(&self) -> &Self::Target {
        &self.hull
    }
}

impl<Inst: std::marker::Copy> ButtonBox<Inst> {
    pub fn new(parent: &Widget<Inst>, path: &'static str,orient: BBOrient,
                align: BAlignment) -> TkResult<Self>
    {
        Ok(Self {
            hull: parent.add_ttk_frame(path)?,
            buttons: HashMap::new(),
            orient: orient,
            alignment: align,
        })
    }
    pub fn add_button<Opts>(&mut self,name: String, options: impl Into<PathOptsWidgets<Opts,()>>) -> TkResult<TtkButton<Inst>>
    where Opts: IntoHomoTuple<TtkButtonOpt>
            + IntoHomoTuple<OptPair>
    {
        let side_ = match self.orient {
            BBOrient::Horizontal => "left",
            BBOrient::Vertical   => "top",
        };
        let anchor_ = match self.orient {
            BBOrient::Horizontal => match self.alignment {
                BAlignment::Left => "w",
                BAlignment::Center => "center",
                BAlignment::Right => "e",
            },
            BBOrient::Vertical   => match self.alignment {
                BAlignment::Left => "n",
                BAlignment::Center => "center",
                BAlignment::Right => "s",
            },
        };
        let widget = self.hull.add_ttk_button(options)?;
        widget.pack(-side(side_) -anchor(anchor_))?;
        self.buttons.insert(name,ButtonWidget::Plain(widget));
        Ok(widget)
    }
    pub fn add_checkbutton<Opts>(&mut self,name: String, options: impl Into<PathOptsWidgets<Opts,()>>) -> TkResult<TtkCheckbutton<Inst>>
    where Opts: IntoHomoTuple<TtkCheckbuttonOpt>
            + IntoHomoTuple<OptPair>
    {
        let side_ = match self.orient {
            BBOrient::Horizontal => "left",
            BBOrient::Vertical   => "top",
        };
        let anchor_ = match self.orient {
            BBOrient::Horizontal => match self.alignment {
                BAlignment::Left => "w",
                BAlignment::Center => "center",
                BAlignment::Right => "e",
            },
            BBOrient::Vertical   => match self.alignment {
                BAlignment::Left => "n",
                BAlignment::Center => "center",
                BAlignment::Right => "s",
            },
        };
        let widget = self.hull.add_ttk_checkbutton(options)?;
        widget.pack(-side(side_) -anchor(anchor_))?;
        self.buttons.insert(name,ButtonWidget::Check(widget));
        Ok(widget)
    }
    pub fn add_radiobutton<Opts>(&mut self,name: String, options: impl Into<PathOptsWidgets<Opts,()>>) -> TkResult<TtkRadiobutton<Inst>>
    where Opts: IntoHomoTuple<TtkRadiobuttonOpt>
            + IntoHomoTuple<OptPair>
    {
        let side_ = match self.orient {
            BBOrient::Horizontal => "left",
            BBOrient::Vertical   => "top",
        };
        let anchor_ = match self.orient {
            BBOrient::Horizontal => match self.alignment {
                BAlignment::Left => "w",
                BAlignment::Center => "center",
                BAlignment::Right => "e",
            },
            BBOrient::Vertical   => match self.alignment {
                BAlignment::Left => "n",
                BAlignment::Center => "center",
                BAlignment::Right => "s",
            },
        };
        let widget = self.hull.add_ttk_radiobutton(options)?;
        widget.pack(-side(side_) -anchor(anchor_))?;
        self.buttons.insert(name,ButtonWidget::Radio(widget));
        Ok(widget)
    }
    pub fn add_menubutton<Opts>(&mut self,name: String, options: impl Into<PathOptsWidgets<Opts,()>>) -> TkResult<TtkMenubutton<Inst>>
    where Opts: IntoHomoTuple<TtkMenubuttonOpt>
            + IntoHomoTuple<OptPair>
    {
        let side_ = match self.orient {
            BBOrient::Horizontal => "left",
            BBOrient::Vertical   => "top",
        };
        let anchor_ = match self.orient {
            BBOrient::Horizontal => match self.alignment {
                BAlignment::Left => "w",
                BAlignment::Center => "center",
                BAlignment::Right => "e",
            },
            BBOrient::Vertical   => match self.alignment {
                BAlignment::Left => "n",
                BAlignment::Center => "center",
                BAlignment::Right => "s",
            },
        };
        let widget = self.hull.add_ttk_menubutton(options)?;
        widget.pack(-side(side_) -anchor(anchor_))?;
        self.buttons.insert(name,ButtonWidget::Menu(widget));
        Ok(widget)
    }
    pub fn invoke(&self,name: String) -> TkResult<()>
    {
        match self.buttons.get(&name) {
            None => Ok(()),
            Some(bw) => {
                match bw {
                    ButtonWidget::Plain(w) => {w.invoke()?;},
                    ButtonWidget::Check(w) => {w.invoke()?;},
                    ButtonWidget::Radio(w) => {w.invoke()?;},
                    ButtonWidget::Menu(w) => {/*w.invoke()?;*/},
                };
                Ok(())
            },
        }
    }
    pub fn setfocus(&self,name: String) -> TkResult<()>
    {
        match self.buttons.get(&name) {
            None => Ok(()),
            Some(bw) => {
                match bw {
                    ButtonWidget::Plain(w) => {w.focus()?;},
                    ButtonWidget::Check(w) => {w.focus()?;},
                    ButtonWidget::Radio(w) => {w.focus()?;},
                    ButtonWidget::Menu(w) => {w.focus()?;},
                };
                Ok(())
            },
        }
    }
}

//trait ButtonOps<T> {
//}
