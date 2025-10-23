// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-10-17 13:05:15
//  Last Modified : <251023.1518>
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
//#![allow(unused_imports)]
#![doc = include_str!("../README.md")]

use time_table::*;
use tk::*;
use tk::cmd::*;
use tcl::*;
pub mod mainwindow;
pub mod ttmainwindow;
pub mod mainframe;
pub mod scrollwindow;
pub mod buttonbox;
pub mod stdmenubar;
pub mod buttonwidget;
use crate::ttmainwindow::*;



fn main()  -> TkResult<()>  {
    let tk = make_tk!()?;
    let root = tk.root();
    let tt: &TimeTableSystem = &TimeTableSystem::old("LJandBS.tt")
                .expect("Failed to open LJandBS.tt");
    let mut mw = TimeTableMainWindow::new(&root)?;
    mw.Chart().buildWholeChart(tt)?;
    //mw?.menu_entryconfigure("file",1,
    //    -command( tclosure!( tk,  || -> TkResult<()> {
    //        let result = tk.eval("tk_getOpenFile -filetypes { {TT .tt TEXT} {All * TEXT} } -title {Enter file to open}")?.to_string();
    //        //eprintln!("result is {}",result);
    //        match TimeTableSystem::old(&result) {
    //            Err(p) => {eprintln!("Error opening {}: {:?}",result,p);},
    //            Ok(t) => tt = t,
    //        };
    //        Ok(())
    //    })))?;
    Ok( main_loop() )
}
