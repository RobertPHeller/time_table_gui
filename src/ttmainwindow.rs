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
//  Last Modified : <251017.2215>
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

//use time_table::*;
use time_table::cab::*;
use tk::*;
use tk::cmd::*;
//use crate::mainwindow::*;
use std::collections::HashMap;
use std::ops::Deref;


pub struct ChartDisplay<Inst: std::marker::Copy + 'static> {
    lheight: f64,
    topofcabs: f64,
    cabheight: f64,
    bottomofcabs: f64,
    numberofcabs: usize,
    cabarray: HashMap<String,f64>,
    topofchart: f64,
    chartheight: f64,
    bottomofchart: f64,
    totallength: f64,
    chartstationoffset: f64,
    topofstorage: f64,
    storagetrackheight: f64,
    bottomofstorage: f64,
    numberofstoragetracks: usize,
    storageoffset: f64,
    stationarray: HashMap<String,f64>, 
    storagearray: HashMap<String,f64>,

    timescale: u32,
    timeinterval: u32,
    labelsize: u32,

    hull: TkCanvas<Inst>,
    
}

impl<Inst: std::marker::Copy + 'static> Deref for ChartDisplay<Inst> {
    type Target = tk::Widget<Inst>;

    fn deref(&self) -> &Self::Target {
        &self.hull
    }
}

impl<Inst:TkInstance> TkPackSlave  for ChartDisplay<Inst> {}
impl<Inst:TkInstance> TkGridSlave  for ChartDisplay<Inst> {}
impl<Inst:TkInstance> TkPlaceSlave for ChartDisplay<Inst> {}

impl<Inst: std::marker::Copy> ChartDisplay<Inst> {
    pub fn new(parent: &Widget<Inst>, timescale: u32, timeinterval: u32, 
                labelsize: u32) -> TkResult<Self> {
        let mut this = Self {lheight: 0.0, topofcabs: 0.0, cabheight: 0.0,
                             bottomofcabs: 0.0, numberofcabs: 0, 
                             cabarray: HashMap::new(), topofchart: 0.0, 
                             chartheight: 0.0, bottomofchart: 0.0, 
                             totallength: 0.0, chartstationoffset: 0.0,
                             topofstorage: 0.0, storagetrackheight: 0.0, 
                             bottomofstorage: 0.0, numberofstoragetracks: 0,
                             storageoffset: 0.0, stationarray: HashMap::new(),
                             storagearray: HashMap::new(),
                             hull: parent.add_canvas( -background("white") -borderwidth(0) -highlightthickness(0) -relief("flat") )?,
                             timescale: if timescale == 0 {1440} else {timescale}, 
                             timeinterval: if timeinterval == 0 {15} else {timeinterval},
                             labelsize: labelsize, };
        //let numIncrs =  (((this.timescale as f64) + (this.timeinterval as f64)) / 
        //                    (this.timeinterval as f64)) as i32;
        //let cwidth = (numIncrs * 20) + this.labelsize as i32 + 20;
        let lab = this.hull.create_text (0.0,0.0, -anchor("nw") -text("T") )?;
        let tags = ( lab.clone(), );
        let bbox = match this.hull.bbox(tags)? {
            None => TkRectangle{ left:0, right:0, top:0, bottom:0 },
            Some(bbox) => bbox,
        };
        //eprintln!("*** ChartDisplay::new(): bbox is {:?}",bbox);
        this.lheight = 1.5 * bbox.bottom as f64;
        Ok ( this )
    }
    pub fn deleteWholeChart(&mut self) {
        let _ = self.hull.delete( ("all", ) );
        self.topofcabs = 0.0;
        self.cabheight = 0.0;
        self.bottomofcabs = 0.0;
        self.numberofcabs = 0;
        self.cabarray.clear();
        self.topofchart = 0.0;
        self.chartheight = 0.0;
        self.bottomofchart = 0.0;
        self.totallength = 0.0;
        self.chartstationoffset = 0.0;
        self.topofstorage = 0.0;
        self.storagetrackheight = 0.0;
        self.bottomofstorage = 0.0;
        self.numberofstoragetracks = 0;
        self.storageoffset = 0.0;
        self.totallength = 0.0;
        self.stationarray.clear();
        self.storagearray.clear();
        self.totallength = 0.0;
    }
    fn _buildTimeLine(&mut self) -> TkResult<()> {
        let numIncrs =  (((self.timescale as f64) + (self.timeinterval as f64)) / 
                            (self.timeinterval as f64)) as i32;
        let cwidth = (numIncrs * 20) + self.labelsize as i32 + 20;
        let scrollWidth = cwidth;
        let topOff = 0;
        for m in (0..=self.timescale).step_by(60) {
            let mx = (self.labelsize as f64) + ( ( ((m as f64) / (self.timeinterval as f64)) * 20.0)) + 4.0;
            self.hull.create_text( mx, 0.0, -anchor("n") -text(format!("{:2}", m / 60)) -tags("TimeLine"))?;
        }
        let bbox = match self.hull.bbox( ("TimeLine", ) )? {
            None => TkRectangle{ left:0, right:0, top:0, bottom:0 },
            Some(bbox) => bbox,
        };
        let scrollHeight = bbox.bottom;
        Ok(self.hull.configure( -scrollregion( (0.0, 0.0, scrollWidth, scrollHeight ) ) )?)
    }
    fn _buildCabs (&mut self) -> TkResult<()> {
        self.hull.delete( ("Cabs", ) )?;
        let bbox = match self.hull.bbox( ("TimeLine", ) )? {
            None => TkRectangle{ left:0, right:0, top:0, bottom:0 },
            Some(bbox) => bbox,
        };
        let topOff = bbox.bottom;
        self.topofcabs = (topOff + 10) as f64;
        self.cabheight = 0.0;
        self.bottomofcabs = self.topofcabs;
        self.numberofcabs = 0;
        for m in (0..=self.timescale).step_by(self.timeinterval as usize) {
            let mx = (self.labelsize as f64) + ( ( ((m as f64) / (self.timeinterval as f64)) * 20.0)) + 4.0;
            let lw = if (m % 60) == 0 {2} else {1};
            self.hull.create_line ( &[(mx,self.topofcabs), (mx,self.bottomofcabs)],
                                    -width(lw) -tags(("Cabs", "Cabs:Tick")))?;
        }
        let r = self.labelsize as f64 + (((self.timescale as f64 / self.timeinterval as f64) * 20.0));
        self.hull.create_line ( &[(self.labelsize as f64,self.topofcabs),(r,self.topofcabs)],
                                -width(2) -tags(("Cabs", "Cabs:Hline")))?;
        self.hull.create_line ( &[(self.labelsize as f64,self.bottomofcabs),(r,self.bottomofcabs)],
                                -width(2) -tags(("Cabs", "Cabs:Hline")))?;
        Ok(())
    }
    pub fn addACab(&mut self, cab: &Cab) -> TkResult<()> {
        let cabyoff: f64;
        if self.numberofcabs == 0 {
            self.numberofcabs = 1;
            self.cabheight = (2.0 * self.lheight) + 20.0;
            self.bottomofcabs = self.topofcabs + self.cabheight;
            cabyoff = self.lheight * 1.75;
        } else {
            self.numberofcabs += 1;
            self.cabheight += self.lheight;
            self.bottomofcabs += self.lheight;
            cabyoff = self.lheight + (self.numberofcabs as f64 + 0.75);
        }
        //self._updateChart()?;
        //self._updateStorageTracks()?;
        //self._updateCabs()?;
        let cabName = cab.Name();
        let cabColor = cab.Color();
        let t2 = String::from("Cabs:Name:")+&cabName;
        self.hull.create_text(0.0,cabyoff + self.topofcabs,
                -text(cabName.clone()) -fill(cabColor.clone()) -tags(("Cabs", t2)) -anchor("w"))?;
        self.cabarray.insert(cabName.clone()+",y",cabyoff+self.topofcabs);
        let r = self.labelsize as f64 + ((self.timescale as f64 / self.timeinterval as f64) * 20.0);
        let t2 = String::from("Cabs:Line:")+&cabName;
        self.hull.create_line(&[(self.labelsize as f64, cabyoff+self.topofcabs),
                                (r,cabyoff+self.topofcabs)],
   -tags( ("Cabs", t2)) -width(4) -fill(cabColor) -stipple("gray50") )?;
        Ok(())
    }
    fn _buildChart(&mut self) -> TkResult<()> {
        Ok(())
    }
}


#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn ChartDisplay_new0 () -> TkResult<()> {
        let tk = make_tk!()?;
        let root = tk.root();
        let temp = ChartDisplay::new(&root,0,0,0)?.pack(-fill("both"));
        Ok( main_loop() )
    }
}

