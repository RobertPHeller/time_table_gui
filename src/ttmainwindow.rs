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
//  Last Modified : <251022.2133>
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

use time_table::*;
use time_table::cab::*;
use time_table::station::*;
use time_table::train::*;
use tk::*;
use tk::cmd::*;
use tk::canvas::{SearchSpec,item_tag,ItemId,ItemTag};
use crate::mainwindow::*;
use crate::scrollwindow::*;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum StationArrayIndex {
    Y(usize),
    Smile(usize),
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum StorageArrayIndex {
    Y(String,String),
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum CabArrayIndex {
    Y(String),
}
pub struct ChartDisplay<Inst: std::marker::Copy + 'static> {
    lheight: f64,
    topofcabs: f64,
    cabheight: f64,
    bottomofcabs: f64,
    numberofcabs: usize,
    cabarray: HashMap<CabArrayIndex,f64>,
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
    stationarray: HashMap<StationArrayIndex,f64>, 
    storagearray: HashMap<StorageArrayIndex,f64>,

    timescale: u32,
    timeinterval: u32,
    labelsize: u32,

    hull: TkCanvas<Inst>,
    
}

impl<Inst: std::marker::Copy + 'static> Deref for ChartDisplay<Inst> {
    type Target = TkCanvas<Inst>;

    fn deref(&self) -> &Self::Target {
        &self.hull
    }
}

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
        let bbox = match this.hull.bbox(tags.clone())? {
            None => TkRectangle{ left:0, right:0, top:0, bottom:0 },
            Some(bbox) => bbox,
        };
        //eprintln!("*** ChartDisplay::new(): bbox is {:?}",bbox);
        this.lheight = 1.5 * bbox.bottom as f64;
        this.hull.delete(tags)?;
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
        self._updateChart()?;
        self._updateStorageTracks()?;
        self._updateCabs()?;
        let cabName = cab.Name();
        let cabColor = cab.Color();
        let t2 = String::from("Cabs:Name:")+&cabName;
        self.hull.create_text(0.0,cabyoff + self.topofcabs,
                -text(cabName.clone()) -fill(cabColor.clone()) -tags(("Cabs", t2)) -anchor("w"))?;
        self.cabarray.insert(CabArrayIndex::Y(cabName.clone()),cabyoff+self.topofcabs);
        let r = self.labelsize as f64 + ((self.timescale as f64 / self.timeinterval as f64) * 20.0);
        let t2 = String::from("Cabs:Line:")+&cabName;
        self.hull.create_line(&[(self.labelsize as f64, cabyoff+self.topofcabs),
                                (r,cabyoff+self.topofcabs)],
          -tags( ("Cabs", t2)) -width(4) -fill(cabColor) -stipple("gray50") )?;
        Ok(())
    }
    fn _buildChart(&mut self) -> TkResult<()> {
        self.hull.delete( ("Chart",) )?;
        let bbox = match self.hull.bbox( ("Cabs", ) )? {
            None => TkRectangle{ left:0, right:0, top:0, bottom:0 },
            Some(bbox) => bbox,
        };
        let topOff = bbox.bottom;
        self.topofchart = topOff as f64 + 10.0;
        self.chartheight = 0.0;
        self.bottomofchart = self.topofchart;
        self.totallength = 0.0;
        for m in (0..=self.timescale).step_by(self.timeinterval as usize) {
            let mx = self.labelsize as f64 + (((m as f64 / self.timeinterval as f64) * 20.0));
            let lw = if (m % 60) == 0 {2} else {1};
            self.hull.create_line(&[(mx,self.topofchart), (mx,self.bottomofchart)], -width(2) -tags( ("Chart", "Chart:Tick") ) )?;
        }
        let r = self.labelsize as f64 + (((self.timescale as f64 / self.timeinterval as f64)) * 20.0);
        self.hull.create_line(&[(self.labelsize as f64,self.topofchart), (r, self.topofchart)], -width(2) -tags( ("Chart", "Chart:Hline") ) )?;
        self.hull.create_line(&[(self.labelsize as f64,self.bottomofchart), (r, self.bottomofchart)],  -width(2) -tags(("Chart", "Chart:Bline")))?;
        self.chartstationoffset = self.topofchart;
        
        Ok(())
    }
    fn _buildStorageTracks(&mut self) -> TkResult<()> {
        self.hull.delete( ("Storage",) )?;
        let bbox = match self.hull.bbox( ("Chart", ) )? {
            None => TkRectangle{ left:0, right:0, top:0, bottom:0 },
            Some(bbox) => bbox,
        };
        let topOff = bbox.bottom;
        self.topofstorage = topOff as f64 + 10.0;
        self.storagetrackheight = 0.0;
        self.bottomofstorage = self.topofstorage;
        self.numberofstoragetracks = 0;
        for m in (0..=self.timescale).step_by(self.timeinterval as usize) {
            let mx = self.labelsize as f64 + (((m as f64 / self.timeinterval as f64) * 20.0));
            let lw = if (m % 60) == 0 {2} else {1};
            self.hull.create_line(&[(mx,self.topofstorage), (mx,self.bottomofstorage)], -width(2) -tags( ("Storage", "Storage:Tick") ) )?;
        }
        let r = self.labelsize as f64 + (((self.timescale as f64 / self.timeinterval as f64)) * 20.0);
        self.hull.create_line(&[(self.labelsize as f64,self.topofstorage), (r, self.topofstorage)], -width(2) -tags( ("Storage", "Storage:Hline") ) )?;
        self.hull.create_line(&[(self.labelsize as f64,self.bottomofstorage), (r, self.bottomofstorage)],  -width(2) -tags(("Storage", "Storage:Bline")))?;
        self.storagearray.clear();
        self.storageoffset = self.topofstorage;
        
        Ok(())
    }
    pub fn addAStation(&mut self,station: &Station, sindex: usize) -> TkResult<()> {
        let mut name: &str = &station.Name();
        let smile = station.SMile();
        if smile > self.totallength {self.totallength = smile;}
        self._updateChart()?;
        self._updateStorageTracks()?;
        let offset = self.topofchart + 20.0;
        let y = offset+(smile * 20.0);
        self.stationarray.insert(StationArrayIndex::Y(sindex),y);
        self.stationarray.insert(StationArrayIndex::Smile(sindex),smile);
        loop {
            let sl = self.hull.create_text(0.0,y, 
                -text(name) 
                -tags( ("Chart", "Station", format!("Station:{}",sindex).as_str()) )
                -anchor("w"))?;
            let tags = ( sl.clone(), );
            let lwid = match self.hull.bbox(tags.clone())? {
                None => 5,
                Some(bbox) => bbox.right + 5,
            };
            if lwid <= self.labelsize as i32 {break;}
            self.hull.delete(tags)?;
            name = &name[0..name.len()-1];
        }
        let namebox_bbox = self.hull.bbox( (format!("Station:{}",sindex).as_str(), ) )?.unwrap();
        let nb = self.hull.create_rectangle(namebox_bbox.left as f64, 
                                            namebox_bbox.top as f64,
                                            namebox_bbox.right as f64, 
                                            namebox_bbox.bottom as f64,
                                            -fill("white") 
                                            -outline("black")
                                             -tags ( ( "Chart",
                                                  "Station",
                                                    format!("Station:namebox:{}",
                                                        sindex).as_str() ) ) )?;
        self.hull.lower( nb, None )?;
        let r = self.labelsize as f64 +
                    (((self.timescale as f64 / self.timeinterval as f64) * 20.0));
        self.hull.create_line(&[(self.labelsize as f64,y), (r,y)],
                            -tags( ("Chart",
                                    "Station",
                                    format!("Station:Line:{}",sindex).as_str() ))
                            -width(2) 
                            -fill("gray50"))?;
        for storage in station.storagetracks() {
            self.addAStorageTrack(station,storage)?;
        }
            
        Ok(())
    }
    pub fn addAStorageTrack(&mut self,station: &Station, track: &StorageTrack)
         -> TkResult<()> 
    {
        let topOff = match self.hull.bbox( ( "Chart", ) )? {
            None => 0,
            Some(bbox) => bbox.bottom,
        };
        self.topofstorage = topOff as f64 + 10.0;
        let storageyoff: f64;
        if self.numberofstoragetracks == 0 {
            self.numberofstoragetracks = 1;
            self.storagetrackheight = (2.0*self.lheight) + 20.0;
            self.bottomofstorage = self.topofstorage + self.storagetrackheight;
            storageyoff = self.lheight as f64 * 1.75;
        } else {
            self.numberofstoragetracks += 1;
            self.storagetrackheight += self.lheight as f64;
            self.bottomofstorage += self.lheight as f64;
            storageyoff = self.lheight as f64 * 
                            (self.numberofstoragetracks as f64 + 0.75);
        } 
        let stationName = station.Name();
        let trackName   = track.Name();
        let nameOnChart = self._formNameOnChart(stationName.as_str(),
                                                trackName.as_str())?;
        let y = storageyoff + self.topofstorage;
        self.storagearray.insert(StorageArrayIndex::Y(stationName.clone(),
                                                      trackName.clone()),y);
        self.hull.create_text(0.0,y, 
            -text(nameOnChart) 
            -tags( ("Storage", 
                    "Storage:track", 
                    format!("Storage:{}:{}",stationName,trackName).as_str() ) )
            -anchor("w"))?;
        let r = self.labelsize as f64 + (((self.timescale as f64 / self.timeinterval as f64) * 20.0));
        self.hull.create_line(&[(self.labelsize as f64, y),
                                (r,y)],
            -tags( ("Storage", "Storage:track",
                    format!("Storage:{}:{}",stationName,trackName).as_str() ))
            -width(4) 
            -stipple("gray50") )?;
        Ok(())
    }
    fn _formNameOnChart(&mut self,sn_: &str,tn_: &str) -> TkResult<String> {
        let mut sn = &sn_[0..];
        let mut tn = &tn_[0..];
        let i = self.hull.create_text(0.0,0.0, 
            -anchor("w") -text(format!("{}:{}",sn,tn).as_str()) )?;
        let mut l1: u32 = self.hull.bbox( (i.clone(), ) )?.unwrap().right as u32;
        self.hull.delete( (i,) )?;
        let i = self.hull.create_text(0.0,0.0, 
            -anchor("w") -text(format!("{}:",sn).as_str()) )?;
        let mut l2: u32 = self.hull.bbox( (i.clone(), ) )?.unwrap().right as u32;
        self.hull.delete( (i,) )?;
        //let i = self.hull.create_text(0.0,0.0, -anchor("w") -text(tn))?;
        //let mut l3 = self.hull.bbox( (i.clone(), ) )?.unwrap().right; 
        //self.hull.delete( (i,) )?; 
        while l1 > self.labelsize && l2 as f64 > (self.labelsize as f64 / 2.0) {
            sn = &sn[0..sn.len()-1];
            let i = self.hull.create_text(0.0,0.0, 
                -anchor("w") -text(format!("{}:{}",sn,tn).as_str()) )?;
            l1 = self.hull.bbox( (i.clone(), ) )?.unwrap().right as u32;
            self.hull.delete( (i,) )?;
            let i = self.hull.create_text(0.0,0.0, 
                -anchor("w") -text(format!("{}:",sn).as_str()) )?;
            l2 = self.hull.bbox( (i.clone(), ) )?.unwrap().right as u32;
            self.hull.delete( (i,) )?;
        }
        while l1 > self.labelsize {
            tn = &tn[0..tn.len()-1];
            let i = self.hull.create_text(0.0,0.0,
                -anchor("w") -text(format!("{}:{}",sn,tn).as_str()) )?;
            l1 = self.hull.bbox( (i.clone(), ) )?.unwrap().right as u32;
            self.hull.delete( (i,) )?;
        }
        Ok(format!("{}:{}",sn,tn))
    }
    pub fn addATrain(&mut self,timetable: &TimeTableSystem, train: &Train)
        -> TkResult<()> {
        let lastX = self.labelsize as f64 + ((self.timescale as f64 / self.timeinterval as f64) * 20.0) + 4.0;
        let firstX = self.labelsize as f64 + 4.0;

        let mut timeX = -1.0;
        let mut stationY = -1.0;
        let mut rStationY = -1.0;
        // The next two variables are not referenced the first time
        // through the loop (because timeX is initialized to -1),
        // but the dumb compiler is afraid they might be referenced
        // when uninitialized.
        let mut color: String = String::from("black");
        let mut cabName: String = String::new();
        let departure = train.Departure();
        let mut oldDepart = -1.0;
        let mut oldSmile = -1.0;
        let speed = train.Speed();
        // The 6 "tempStringN" variables are used to hold the memory for
        // the slices in the tag tuples.  They are not otherwised used,
        // Really the compiler should figure this out, but it just complains.
        let tempString1 = format!("Chart:Train:{}",train.Number());
        let tempString2 = format!("Train:{}",train.Number());
        let trtags = ("Chart", "Chart:Train", 
                      tempString1.as_str(), tempString2.as_str() );
        let tempString3 = format!("Cabs:Train:{}",train.Number());
        let tempString4 = format!("Train:{}",train.Number());
        let cabtags = ("Cabs", "Cabs:Train",
                       tempString3.as_str(), tempString4.as_str() );
        let tempString5 = format!("Storage:Train:{}",train.Number());
        let tempString6 = format!("Train:{}",train.Number());
        let stortags = ("Storage", "Storage:track", "Storage:Train",
                        tempString5.as_str(),tempString6.as_str());
        for stop in train.StopIter() {
            let sindex = stop.StationIndex();
            let station = timetable.IthStation(sindex).unwrap();
            let smile = station.SMile();
            let rSindex = station.DuplicateStationIndex();
            let (rStation,rsmile,newRStationY) =
                match rSindex {
                    None => (None,-1.0,-1.0),
                    Some(rsind) => {
                        let rStation = timetable.IthStation(rsind).unwrap();
                        let rsmile = rStation.SMile();
                        let newRStationY = self.stationarray.get(&StationArrayIndex::Y(rsind)).unwrap();
                        (Some(rStation),rsmile,*newRStationY)
                    },
            };
            let departcab = stop.TheCab();
            let newcolor: String;
            let newname: String;
            //let newColor: &str;
            //let newCabName: &str;
            match departcab {
                None => {
                    newcolor = String::from("black");
                    newname = String::new();
                },                    
                Some(cab) => {
                    newcolor = cab.Color();
                    newname =  cab.Name();
                },
            };
            let newStationY = self.stationarray.get(&StationArrayIndex::Y(sindex)).unwrap();
            let arrival: f64 = if oldDepart >= 0.0 {
                oldDepart + (smile - oldSmile).abs() * (speed as f64 / 60.0)
            } else {
                departure as f64
            };
            let storage: Option<&StorageTrack>;
            let rstorage: Option<&StorageTrack>;
            match stop.Flag() {
                StopFlagType::Origin => {
                    let depart = departure as f64;
                    storage = station.FindTrackTrainIsStoredOn(train.Number(),depart,depart);
                    if rStation.is_some() {
                        rstorage = rStation.unwrap().FindTrackTrainIsStoredOn(train.Number(),depart,depart);
                    } else {
                        rstorage = None;
                    }
                },
                StopFlagType::Terminate => {
                    storage = station.FindTrackTrainIsStoredOn(train.Number(),arrival,arrival);
                    if rStation.is_some() {
                        rstorage = rStation.unwrap().FindTrackTrainIsStoredOn(train.Number(),arrival,arrival);
                    } else {
                        rstorage = None;
                    }
                },
                StopFlagType::Transit => {
                    storage = None;
                    rstorage = None;
                },
            };
            if storage.is_some() {
                let stationName = station.Name();
                let trackName   = storage.unwrap().Name();
                let sy = *self.storagearray.get(&StorageArrayIndex::Y(stationName,trackName)).unwrap();
                let occupiedA = storage.unwrap().IncludesTime(arrival);
                let occupiedD = storage.unwrap().IncludesTime(departure as f64);
                if occupiedA.is_some() &&
                   occupiedA.unwrap().TrainNum() == train.Number() {
                    let from = occupiedA.unwrap().From();
                    let to   = occupiedA.unwrap().Until();
                    let fromX = self.labelsize as f64 +
                                    ((from / self.timeinterval as f64) * 20.0) + 4.0;
                    let toX   = self.labelsize as f64 +
                                    ((to / self.timeinterval as f64) * 20.0) + 4.0;
                    if toX > fromX {
                        self.hull.create_line(&[(fromX,sy),(toX,sy)],
                            -fill(newcolor.as_str()) -width(8) -tags(stortags))?;
                    } else {
                        self.hull.create_line(&[(fromX,sy),(lastX,sy)],
                            -fill(newcolor.as_str()) -width(8) -tags(stortags))?;
                        self.hull.create_line(&[(firstX,sy),(toX,sy)],
                            -fill(newcolor.as_str()) -width(8) -tags(stortags))?;
                    }
                }
                if occupiedD.is_some() &&
                   occupiedD.unwrap().TrainNum2() == train.Number() {
                    let from = occupiedD.unwrap().From();
                    let to   = occupiedD.unwrap().Until();
                    let fromX = self.labelsize as f64 +
                                    ((from / self.timeinterval as f64) * 20.0) + 4.0;
                    let toX   = self.labelsize as f64 +
                                    ((to / self.timeinterval as f64) * 20.0) + 4.0;
                    if toX > fromX {
                        self.hull.create_line(&[(fromX,sy),(toX,sy)],
                            -fill(newcolor.as_str()) -width(8) -tags(stortags))?;
                    } else {
                        self.hull.create_line(&[(fromX,sy),(lastX,sy)],
                            -fill(newcolor.as_str()) -width(8) -tags(stortags))?;
                        self.hull.create_line(&[(firstX,sy),(toX,sy)],
                            -fill(newcolor.as_str()) -width(8) -tags(stortags))?;
                    }
                }
            }
            if rstorage.is_some() {
                let stationName = rStation.unwrap().Name();
                let trackName   = rstorage.unwrap().Name();
                let sy = *self.storagearray.get(&StorageArrayIndex::Y(stationName,trackName)).unwrap();
                let occupiedA = rstorage.unwrap().IncludesTime(arrival);
                let occupiedD = rstorage.unwrap().IncludesTime(departure as f64);
                if occupiedA.is_some() &&
                   occupiedA.unwrap().TrainNum() == train.Number() {
                    let from = occupiedA.unwrap().From();
                    let to   = occupiedA.unwrap().Until();
                    let fromX = self.labelsize as f64 +
                                    ((from / self.timeinterval as f64) * 20.0) + 4.0;
                    let toX   = self.labelsize as f64 +
                                    ((to / self.timeinterval as f64) * 20.0) + 4.0;
                    if toX > fromX {
                        self.hull.create_line(&[(fromX,sy),(toX,sy)],
                            -fill(newcolor.as_str()) -width(8) -tags(stortags))?;
                    } else {
                        self.hull.create_line(&[(fromX,sy),(lastX,sy)],
                            -fill(newcolor.as_str()) -width(8) -tags(stortags))?;
                        self.hull.create_line(&[(firstX,sy),(toX,sy)],
                            -fill(newcolor.as_str()) -width(8) -tags(stortags))?;
                    }
                }
                if occupiedD.is_some() &&
                   occupiedD.unwrap().TrainNum2() == train.Number() {
                    let from = occupiedD.unwrap().From();
                    let to   = occupiedD.unwrap().Until();
                    let fromX = self.labelsize as f64 +
                                    ((from / self.timeinterval as f64) * 20.0) + 4.0;
                    let toX   = self.labelsize as f64 +
                                    ((to / self.timeinterval as f64) * 20.0) + 4.0;
                    if toX > fromX {
                        self.hull.create_line(&[(fromX,sy),(toX,sy)],
                            -fill(newcolor.as_str()) -width(8) -tags(stortags))?;
                    } else {
                        self.hull.create_line(&[(fromX,sy),(lastX,sy)],
                            -fill(newcolor.as_str()) -width(8) -tags(stortags))?;
                        self.hull.create_line(&[(firstX,sy),(toX,sy)],
                            -fill(newcolor.as_str()) -width(8) -tags(stortags))?;
                    }
                }
            }
            let mut newTimeX = self.labelsize as f64 + ((arrival / self.timeinterval as f64) * 20.0) + 4.0;
            if timeX >= 0.0 {
                if newTimeX > timeX {
                    self.hull.create_line(
                        &[(timeX,stationY),(newTimeX,*newStationY)],
                        -fill(color.clone()) -width(4) -tags(trtags))?;
                    if rStationY >= 0.0 && newRStationY >= 0.0 {
                        self.hull.create_line(
                            &[(timeX,rStationY),(newTimeX,newRStationY)],
                            -fill(color.clone()) -width(4) -tags(trtags))?;
                    }
                    match self.cabarray.get(&CabArrayIndex::Y(cabName.clone())) {
                        None => (),
                        Some(cy) => {
                            self.hull.create_line(
                                &[(timeX,*cy),(newTimeX,*cy)],
                                -fill(color.clone())   -width(8) -tags(cabtags))?;
                        },
                    }
                } else {
                    let unwrapNX = newTimeX - lastX;
                    let slope = (newStationY - stationY) as f64 / (unwrapNX - timeX) as f64;
                    let midY = stationY + (slope * (lastX - timeX));
                    self.hull.create_line(&[(timeX,stationY),(lastX,midY)],
                                -fill(color.clone()) -width(4) -tags(trtags) )?;
                    self.hull.create_line(&[(firstX,midY),(newTimeX,*newStationY)],
                                -fill(color.clone()) -width(4) -tags(trtags) )?;
                    if rStationY >= 0.0 && newRStationY >= 0.0 {
                        let slope = (newRStationY - rStationY) as f64 / 
                                    (unwrapNX - timeX) as f64;
                        let midY = rStationY + (slope * (lastX - timeX));
                        self.hull.create_line(&[(timeX,rStationY),
                                                (lastX,midY)],
                                -fill(color.clone()) -width(4) -tags(trtags) )?;
                        self.hull.create_line(&[(firstX,midY),
                                                (newTimeX,newRStationY)],
                                -fill(color.clone()) -width(4) -tags(trtags) )?;
                    }
                    match self.cabarray.get(&CabArrayIndex::Y(cabName.clone())) {
                        None => (),
                        Some(cy) => {
                            self.hull.create_line(
                                &[(timeX,*cy),(newTimeX,*cy)],
                                -fill(color.clone())   -width(8) -tags(cabtags))?;
                        },
                    }
                }
            }
            timeX = newTimeX;
            cabName = newname;
            color = newcolor;
            stationY = *newStationY;
            rStationY = newRStationY;
            let depart = stop.Departure(arrival);
            if depart > arrival {
                let (cy, dontdrawcab): (f64, bool) = 
                    match self.cabarray.get(&CabArrayIndex::Y(cabName.clone())) {
                        None => (0.0, true),
                        Some(cy) => (*cy, false),
                };
                newTimeX = self.labelsize as f64 + ((depart / self.timeinterval as f64) * 20.0) + 4.0;
                if newTimeX > timeX {
                    self.hull.create_line(
                        &[(timeX,stationY),(newTimeX,stationY)],
                        -fill(color.clone()) -width(4) -tags(trtags) )?;
                    if rStationY >= 0.0 {
                        self.hull.create_line(
                            &[(timeX,rStationY),(newTimeX,rStationY)], 
                            -fill(color.clone()) -width(4) -tags(trtags) )?;
                    }
                    if !dontdrawcab {
                        self.hull.create_line(
                            &[(timeX,cy),(newTimeX,cy)],
                            -fill(color.clone())   -width(8) -tags(cabtags) )?;
                    }
                } else {
                    self.hull.create_line(
                        &[(timeX,stationY),(lastX,stationY)],
                        -fill(color.clone()) -width(4) -tags(trtags) )?;
                    self.hull.create_line(
                        &[(firstX,stationY),(newTimeX,stationY)],
                        -fill(color.clone()) -width(4) -tags(trtags) )?;
                    if rStationY >= 0.0 {
                        self.hull.create_line(
                            &[(timeX,rStationY),(lastX,rStationY)], 
                            -fill(color.clone()) -width(4) -tags(trtags) )?;
                        self.hull.create_line(
                            &[(firstX,rStationY),(newTimeX,rStationY)], 
                            -fill(color.clone()) -width(4) -tags(trtags) )?;
                    }
                    if !dontdrawcab {
                        self.hull.create_line(
                            &[(timeX,cy),(lastX,cy)],
                            -fill(color.clone())   -width(8) -tags(cabtags) )?;
                        self.hull.create_line(
                            &[(firstX,cy),(newTimeX,cy)],
                            -fill(color.clone())   -width(8) -tags(cabtags) )?;
                    }
                }
                let storage = station.FindTrackTrainIsStoredOn(train.Number(),arrival,depart);
                if storage.is_some() {
                    let stationName = station.Name();
                    let trackName   = storage.unwrap().Name();
                    let sy = *self.storagearray.get(&StorageArrayIndex::Y(stationName,trackName)).unwrap();
                    let occupiedA = storage.unwrap().IncludesTime(arrival);
                    let occupiedD = storage.unwrap().IncludesTime(depart);
                    if occupiedA.is_some() &&
                       occupiedA.unwrap().TrainNum() == train.Number() {
                        let from = occupiedA.unwrap().From();
                        let to   = occupiedA.unwrap().Until();
                        let fromX = self.labelsize as f64 +
                                        ((from / self.timeinterval as f64) * 20.0) + 4.0;
                        let toX   = self.labelsize as f64 +
                                        ((to / self.timeinterval as f64) * 20.0) + 4.0;
                        if toX > fromX {
                            self.hull.create_line(&[(fromX,sy),(toX,sy)],
                                -fill(color.as_str()) -width(8) -tags(stortags))?;
                        } else {
                            self.hull.create_line(&[(fromX,sy),(lastX,sy)],
                                -fill(color.as_str()) -width(8) -tags(stortags))?;
                            self.hull.create_line(&[(firstX,sy),(toX,sy)],
                                -fill(color.as_str()) -width(8) -tags(stortags))?;
                        }
                    }
                    if occupiedD.is_some() &&
                       occupiedA != occupiedD &&
                       occupiedD.unwrap().TrainNum() == train.Number() {
                        let from = occupiedD.unwrap().From();
                        let to   = occupiedD.unwrap().Until();
                        let fromX = self.labelsize as f64 +
                                        ((from / self.timeinterval as f64) * 20.0) + 4.0;
                        let toX   = self.labelsize as f64 +
                                        ((to / self.timeinterval as f64) * 20.0) + 4.0;
                        if toX > fromX {
                            self.hull.create_line(&[(fromX,sy),(toX,sy)],
                                -fill(color.as_str()) -width(8) -tags(stortags))?;
                        } else {
                            self.hull.create_line(&[(fromX,sy),(lastX,sy)],
                                -fill(color.as_str()) -width(8) -tags(stortags))?;
                            self.hull.create_line(&[(firstX,sy),(toX,sy)],
                                -fill(color.as_str()) -width(8) -tags(stortags))?;
                        }
                    }
                        
                }
                if rStation.is_some() {
                    let storage = rStation
                                    .unwrap()
                                    .FindTrackTrainIsStoredOn(train.Number(),
                                                              arrival,
                                                              depart);
                    if storage.is_some() {
                        let stationName = rStation.unwrap().Name();
                        let trackName   = storage.unwrap().Name();
                        let sy = *self.storagearray.get(&StorageArrayIndex::Y(stationName,trackName)).unwrap();
                        let occupiedA = storage.unwrap().IncludesTime(arrival);
                        let occupiedD = storage.unwrap().IncludesTime(depart);
                        if occupiedA.is_some() &&
                           occupiedA.unwrap().TrainNum() == train.Number() {
                            let from = occupiedA.unwrap().From();
                            let to   = occupiedA.unwrap().Until();
                            let fromX = self.labelsize as f64 +
                                            ((from / self.timeinterval as f64) * 20.0) + 4.0;
                            let toX   = self.labelsize as f64 +
                                            ((to / self.timeinterval as f64) * 20.0) + 4.0;
                            if toX > fromX {
                                self.hull.create_line(&[(fromX,sy),(toX,sy)],
                                    -fill(color.as_str()) -width(8) -tags(stortags))?;
                            } else {
                                self.hull.create_line(&[(fromX,sy),(lastX,sy)],
                                    -fill(color.as_str()) -width(8) -tags(stortags))?;
                                self.hull.create_line(&[(firstX,sy),(toX,sy)],
                                    -fill(color.as_str()) -width(8) -tags(stortags))?;
                            }
                        }
                        if occupiedD.is_some() &&
                           occupiedA != occupiedD &&
                           occupiedD.unwrap().TrainNum() == train.Number() {
                            let from = occupiedD.unwrap().From();
                            let to   = occupiedD.unwrap().Until();
                            let fromX = self.labelsize as f64 +
                                            ((from / self.timeinterval as f64) * 20.0) + 4.0;
                            let toX   = self.labelsize as f64 +
                                            ((to / self.timeinterval as f64) * 20.0) + 4.0;
                            if toX > fromX {
                                self.hull.create_line(&[(fromX,sy),(toX,sy)],
                                    -fill(color.as_str()) -width(8) -tags(stortags))?;
                            } else {
                                self.hull.create_line(&[(fromX,sy),(lastX,sy)],
                                    -fill(color.as_str()) -width(8) -tags(stortags))?;
                                self.hull.create_line(&[(firstX,sy),(toX,sy)],
                                    -fill(color.as_str()) -width(8) -tags(stortags))?;
                            }
                        }
                    }
                }
                timeX = newTimeX;
            }
            oldDepart = depart;
            oldSmile  = smile;
        }
        Ok(())
    }
    fn mx2minutes(&self, mx: i32) -> TkResult<f64> {
        let cx = self.hull.canvasx(mx as f64,None)?;
        let time = ((cx - self.labelsize as f64 - 4.0) / 20.0) * self.timeinterval as f64;
        Ok(time)
    }
    pub fn deleteTrain(&mut self, trainnumber: &str) -> TkResult<()> {
        self.hull.delete( (format!("Train:{}",trainnumber), ) )?;
        Ok(())
    }
    fn _updateChart(&mut self) -> TkResult<()> {
        let topOff = match self.hull.bbox( ("Cabs", ) )? {
            None => 0,
            Some(bbox) => bbox.bottom,
        };
        self.topofchart = (topOff + 10) as f64;
        let ty: f64;
        let by: f64;
        if self.totallength == 0.0 {
            self.bottomofchart = self.topofchart;
            ty = self.topofchart;
            by = self.bottomofchart;
            self.chartheight = 0.0;
        } else {
            self.chartheight = (self.totallength * 20.0) + 20.0;
            self.bottomofchart = self.topofchart + self.chartheight + 20.0;
            ty = self.topofchart + 10.0;
            by = self.bottomofchart - 10.0;
        }
        let taglist = self.hull.find(SearchSpec::WithTag( 
                        item_tag( "Chart:Tick" ).into()))?;
        for tick in taglist.get_elements()?
                            .map( |obj| ItemId(obj.to_string())) {
            let mut coords = self.hull.coords(tick.clone())?.get_elements()?
                        .map( |obj| obj.as_f64() )
                        .collect::<Vec<_>>();
            coords[1] = ty;
            coords[3] = by;
            self.hull.set_coords(tick,coords.into())?;
        }
        let mut tick = ItemTag ( "Chart:Hline".to_owned() ) ;
        let mut coords = self.hull.coords( tick.clone() )?
                        .get_elements()?
                        .map( |obj| obj.as_f64() )
                        .collect::<Vec<_>>(); 
        coords[1] = ty;
        coords[3] = by;
        self.hull.set_coords(tick,coords.into())?;
        tick =  ItemTag ( "Chart:Bline".to_owned() );
        coords = self.hull.coords(tick.clone())?                     
                        .get_elements()?
                        .map( |obj| obj.as_f64() )
                        .collect::<Vec<_>>();
        coords[1] = ty;
        coords[3] = by;
        self.hull.set_coords(tick,coords.into())?;
        let mut sindexes: Vec<usize> = Vec::new();
        for k in self.stationarray.keys() {
            match k {
                StationArrayIndex::Smile(unused) => (),
                StationArrayIndex::Y(sindex) => sindexes.push(*sindex),
            };
        }
        let offset = self.topofchart + 20.0;
        for sindex in sindexes.iter() {
            let stationIndex = StationArrayIndex::Y(*sindex);
            let smile = *self.stationarray.get(&StationArrayIndex::Smile(*sindex)).unwrap();
            let sy = offset + (smile * 20.0);
            let elt = self.stationarray.get_mut(&stationIndex).unwrap();
            *elt = sy;
            let mut thetag = format!("Station:{}",sindex);
            let mut theItemTag = ItemTag(thetag.as_str().to_owned());
            let mut coords = self.hull.coords(theItemTag.clone())?
                .get_elements()?
                .map( |obj| obj.as_f64() )
                .collect::<Vec<_>>();
            coords[1] = sy;
            self.hull.set_coords(theItemTag,coords.into())?;
            thetag = format!("Station:Line:{}",sindex);
            theItemTag = ItemTag(thetag.as_str().to_owned());
            coords = self.hull.coords(theItemTag.clone())?
                .get_elements()?
                .map( |obj| obj.as_f64() )
                .collect::<Vec<_>>(); 
            coords[1] = sy;
            coords[3] = sy;
            self.hull.set_coords(theItemTag.clone(),coords.into())?;
            thetag = format!("Station:namebox:{}",sindex);
            let bbox = self.hull.bbox( (thetag.as_str(), ) )?.unwrap();
            let mut coords: Vec<f64> = Vec::new();
            coords.push(bbox.left as f64);
            coords.push(bbox.top as f64);
            coords.push(bbox.right as f64);
            coords.push(bbox.bottom as f64);
            self.hull.set_coords(theItemTag,coords.into())?;
        }
        Ok(())
    }
    fn _updateStorageTracks(&mut self) -> TkResult<()> {
        Ok(())
    }
    fn _updateCabs(&mut self) -> TkResult<()> {
        Ok(())
    }
    pub fn buildWholeChart(&mut self, timetable: &TimeTableSystem) -> TkResult<()> {
        Ok(())
    }
}


pub struct TimeTable<Inst: std::marker::Copy + 'static> {
    FocusNowhere: TkCanvas<Inst>,
    Main:         MainWindow<Inst>,
    SysConfigFile: String,
    MainWindow:   ScrollWindow<Inst>,
    ChartDisplay: ChartDisplay<Inst>,
}

impl<Inst: std::marker::Copy> TimeTable<Inst> {
}

impl<Inst: std::marker::Copy + 'static> Deref for TimeTable<Inst> {
    type Target = MainWindow<Inst>;

    fn deref(&self) -> &Self::Target {
        &self.Main
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

