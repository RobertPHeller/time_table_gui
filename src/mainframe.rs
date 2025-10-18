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
//  Last Modified : <251017.2331>
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
use std::ops::Deref;
use crate::scrollwindow::*;
use crate::buttonbox::*;

pub struct MainFrame<Inst: std::marker::Copy + 'static> {
    hull: TtkFrame<Inst>,
}

impl<Inst: std::marker::Copy> MainFrame<Inst> {
}

impl<Inst: std::marker::Copy + 'static> Deref for MainFrame<Inst> {
    type Target = tk::Widget<Inst>;

    fn deref(&self) -> &Self::Target {
        &self.hull
    }
}

