// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-10-17 23:23:12
//  Last Modified : <251021.1116>
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
use std::os::raw::c_double;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct HsbFlags {
    pub present: bool,
    pub packed: bool,
    pub auto: bool,
    pub row: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct VsbFlags {
    pub present: bool,
    pub packed: bool,
    pub auto: bool,
    pub column: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollOpt {None, Both, Vertical, Horizontal}

#[derive(Debug, Clone, Copy, PartialEq)] 
pub enum SidesOpt {Ne, En, Nw, Wn, Se, Es, Sw, Ws}

enum ScrollableWidget<Inst: std::marker::Copy + 'static> {
    None,
    Canvas(TkCanvas<Inst>),
    Text(TkText<Inst>),
    ListBox(TkListbox<Inst>),
    TreeView(TtkTreeview<Inst>),
}

pub struct ScrollWindow<Inst: std::marker::Copy + 'static> {
    //tk_inst: Tk<Inst>,
    // Hull
    hull: TtkFrame<Inst>,
    // Components
    hscroll: TtkScrollbar<Inst>,
    vscroll: TtkScrollbar<Inst>,
    // variables
    realized: bool,
    hsb: HsbFlags,
    vsb: VsbFlags,
    widget: ScrollableWidget<Inst>,
    changed: bool,
    hlock: bool,
    vlock: bool,
}

pub trait Setwidget<T> {
    fn setwidget(&mut self, wid: T) -> TkResult<()>;
}

impl<Inst: std::marker::Copy + 'static> Setwidget<TkCanvas<Inst>> for ScrollWindow<Inst> {
    fn setwidget(&mut self, wid: TkCanvas<Inst>) -> TkResult<()>
    {
        self.setwidget_common(ScrollableWidget::Canvas(wid))
    }
}

impl<Inst: std::marker::Copy + 'static> Setwidget<TkText<Inst>> for ScrollWindow<Inst> {
    fn setwidget(&mut self, wid: TkText<Inst>) -> TkResult<()>
    {
        self.setwidget_common(ScrollableWidget::Text(wid))
    }
}

impl<Inst: std::marker::Copy + 'static> Setwidget<TkListbox<Inst>> for ScrollWindow<Inst> {
    fn setwidget(&mut self, wid: TkListbox<Inst>) -> TkResult<()>
    {
        self.setwidget_common(ScrollableWidget::ListBox(wid))
    }
}

impl<Inst: std::marker::Copy + 'static> Setwidget<TtkTreeview<Inst>> for ScrollWindow<Inst> {
    fn setwidget(&mut self, wid: TtkTreeview<Inst>) -> TkResult<()>
    {
        self.setwidget_common(ScrollableWidget::TreeView(wid))
    }
}

impl<Inst: std::marker::Copy + 'static> Deref for ScrollWindow<Inst> {
     type Target = TtkFrame<Inst>;

    fn deref(&self) -> &Self::Target {
        &self.hull
    }
}

impl<Inst: std::marker::Copy> ScrollWindow<Inst> {
    //fn tk(&self) -> Tk<Inst> {self.tk_inst}
    pub fn new(/*tk_: Tk<Inst>,*/parent: &Widget<Inst>, path: &'static str, 
                scrollbar: ScrollOpt,
                auto: ScrollOpt, sides: SidesOpt, size: u32, ipad: u32, 
                managed: bool) -> TkResult<Self>
    {
        let hull =  parent.add_ttk_frame(path)?;
        let hscroll = hull.add_ttk_scrollbar("hscroll" 
                                            -takefocus(true) 
                                            -orient("horizontal"))?;
        let vscroll = hull.add_ttk_scrollbar("vscroll"
                                            -takefocus(true)
                                            -orient("vertical"))?;
        let mut this = Self {/*tk_inst: tk_,*/hull: hull, hscroll: 
                            hscroll, vscroll: vscroll,
                            realized: true, 
                            hsb: HsbFlags {present: false, 
                                            packed: false,
                                            auto: false, 
                                            row: 0},
                            vsb: VsbFlags {present: false,
                                            packed: false,
                                            auto: false,
                                            column: 0},
                            widget: ScrollableWidget::None,
                            changed: false, hlock: false, vlock: false};
        this._setdata(scrollbar,ScrollOpt::None,sides);
        if managed {
            this.hsb.packed = this.hsb.present;
            this.vsb.packed = this.vsb.present;
        } else {
            this.hsb.packed = false;
            this.vsb.packed = false;
        }
        if this.hsb.packed {
            this.hscroll.grid(-column(1) -row(this.hsb.row) -sticky("ew") 
                                -ipady(ipad as i32))?; 
        }
        if this.vsb.packed {
            this.vscroll.grid(-column(this.vsb.column) -row(1) -sticky("ns")
                                 -ipadx(ipad as i32))?;
        }
        this.hull.grid_columnconfigure(1, -weight(1))?;
        this.hull.grid_rowconfigure(1, -weight(1))?; 
        //this._setupRealize()?;
        Ok(this)
    }
    //fn _setupRealize(&self) -> TkResult<()>
    //{
    //    self.hull.bind( event::configure(), 
    //                    tclosure!(self.tk(),|self| -> TkResult<()> {self._realize()}))?;
    //    Ok(())
    //}
    pub fn getframe(&self) -> TtkFrame<Inst> {self.hull}
    fn setwidget_common(&mut self,wid: ScrollableWidget<Inst>) -> TkResult<()>
    {
        match self.widget {
            ScrollableWidget::None => (),
            ScrollableWidget::Canvas(w) => {
                w.grid_remove()?;
                w.configure(-xscrollcommand("") -yscrollcommand(""))?;
            },
            ScrollableWidget::Text(w) => {
                w.grid_remove()?;
                w.configure(-xscrollcommand("") -yscrollcommand(""))?;
            },
            ScrollableWidget::ListBox(w) => {
                w.grid_remove()?;
                w.configure(-xscrollcommand("") -yscrollcommand(""))?;
            },
            ScrollableWidget::TreeView(w) => {
                w.grid_remove()?;
                w.configure(-xscrollcommand("") -yscrollcommand(""))?;
            },
        };
        self.widget = wid;
        match self.widget {
            ScrollableWidget::None => (),
            ScrollableWidget::Canvas(w) => {
                w.grid(-in_(self.hull) -row(1) -column(1) -sticky("news"))?;
                self.hscroll.configure(-command(String::from(w.path())+" xview"))?;
                self.vscroll.configure(-command(String::from(w.path())+" yview"))?;
                //w.configure(-xscrollcommand(
                //    tclosure!(self.tk(),
                //        |vmin:c_double, vmax:c_double| -> TkResult<()> {
                //            self._set_hscroll(vmin, vmax)
                //        }
                //    ))
                //)?;
                w.configure(-xscrollcommand(String::from(self.hscroll.path())+" set"))?;
                //w.configure(-yscrollcommand(
                //    tclosure!(self.tk(),
                //        |vmin:c_double, vmax:c_double| -> TkResult<()> {
                //            self._set_vscroll(vmin, vmax)
                //        }
                //    ))
                //)?;
                w.configure(-yscrollcommand(String::from(self.vscroll.path())+" set"))?;
            },
            ScrollableWidget::Text(w) => {
                w.grid(-in_(self.hull) -row(1) -column(1) -sticky("news"))?;
                self.hscroll.configure(-command(String::from(w.path())+" xview"))?;
                self.vscroll.configure(-command(String::from(w.path())+" yview"))?;
                //w.configure(-xscrollcommand(
                //    tclosure!(self.tk(),
                //        |vmin:c_double, vmax:c_double| -> TkResult<()> {
                //            self._set_hscroll(vmin, vmax)
                //        }
                //    ))
                //)?;
                w.configure(-xscrollcommand(String::from(self.hscroll.path())+" set"))?;
                //w.configure(-yscrollcommand(
                //    tclosure!(self.tk(),
                //        |vmin:c_double, vmax:c_double| -> TkResult<()> {
                //            self._set_vscroll(vmin, vmax)
                //        }
                //    ))
                //)?;
                w.configure(-yscrollcommand(String::from(self.vscroll.path())+" set"))?;
            },
            ScrollableWidget::ListBox(w) => {
                w.grid(-in_(self.hull) -row(1) -column(1) -sticky("news"))?;
                self.hscroll.configure(-command(String::from(w.path())+" xview"))?;
                self.vscroll.configure(-command(String::from(w.path())+" yview"))?;
                //w.configure(-xscrollcommand(
                //    tclosure!(self.tk(),
                //        |vmin:c_double, vmax:c_double| -> TkResult<()> {
                //            self._set_hscroll(vmin, vmax)
                //        }
                //    ))
                //)?;
                w.configure(-xscrollcommand(String::from(self.hscroll.path())+" set"))?;
                //w.configure(-yscrollcommand(
                //    tclosure!(self.tk(),
                //        |vmin:c_double, vmax:c_double| -> TkResult<()> {
                //            self._set_vscroll(vmin, vmax)
                //        }
                //    ))
                //)?;
                w.configure(-yscrollcommand(String::from(self.vscroll.path())+" set"))?;
            },
            ScrollableWidget::TreeView(w) => {
                w.grid(-in_(self.hull) -row(1) -column(1) -sticky("news"))?;
                self.hscroll.configure(-command(String::from(w.path())+" xview"))?;
                self.vscroll.configure(-command(String::from(w.path())+" yview"))?;
                //w.configure(-xscrollcommand(
                //    tclosure!(self.tk(),
                //        |vmin:c_double, vmax:c_double| -> TkResult<()> {
                //            self._set_hscroll(vmin, vmax)
                //        }
                //    ))
                //)?;
                w.configure(-xscrollcommand(String::from(self.hscroll.path())+" set"))?;
                //w.configure(-yscrollcommand(
                //    tclosure!(self.tk(),
                //        |vmin:c_double, vmax:c_double| -> TkResult<()> {
                //            self._set_vscroll(vmin, vmax)
                //        }
                //    ))
                //)?;
                w.configure(-yscrollcommand(String::from(self.vscroll.path())+" set"))?;
            },
        };
        Ok(())
    }
    fn _set_hscroll(&self,vmin:c_double, vmax:c_double) -> TkResult<()>
    {
        Ok(())
    }
    fn _set_vscroll(&self,vmin:c_double, vmax:c_double) -> TkResult<()>
    {
        Ok(())
    }
    fn _setdata(&mut self,scrollbar: ScrollOpt, auto: ScrollOpt, 
                sides: SidesOpt)
    {
        self.hsb.present = match scrollbar {
            ScrollOpt::Horizontal | ScrollOpt::Both => true,
            _ => false,
        };
        self.hsb.auto = match auto {
            ScrollOpt::Horizontal | ScrollOpt::Both => true,
            _ => false,
        };
        self.hsb.row = match sides {
            SidesOpt::Ne | SidesOpt::En | SidesOpt::Nw | SidesOpt::Wn => 0,
            _ => 2,
        };

        self.vsb.present = match scrollbar {
            ScrollOpt::Vertical | ScrollOpt::Both => true,
            _ => false,
        };
        self.vsb.auto = match auto {
            ScrollOpt::Vertical | ScrollOpt::Both => true,
            _ => false,
        };
        self.vsb.column = match sides {
            SidesOpt::Nw | SidesOpt::Wn | SidesOpt::Sw | SidesOpt::Ws => 0,
            _ => 2,
        };
    }
    //fn _realize(&mut self) ->TkResult<()> {
    //    self.hull.bind(event::configure(),())?;
    //    self.realized = true;
    //    Ok(())
    //}
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test] 
    fn ScrollWindow_new () -> TkResult<()> {
        let tk = make_tk!()?;
        let root = tk.root();
        let mut scroll = ScrollWindow::new(&root,"scroll",ScrollOpt::Both,
                                        ScrollOpt::None,SidesOpt::Se,
                                        1,0,true)?;
        scroll.pack(-expand("yes") -fill("both"))?;
        let frame = scroll.getframe();
        let canvas = frame.add_canvas("c" -width(600) -height(400))?;
        scroll.setwidget(canvas)?;
        canvas.configure( -scrollregion( (-600, -400, 600, 400) ))?;
        canvas.create_oval( -10.0, -10.0, 10.0, 10.0, -fill("red"))?;
        let button = root.add_ttk_button("button" -text("exit")  -command("exit"))?
                .pack(-fill("x"))?;
        Ok( main_loop() )
    }
}

