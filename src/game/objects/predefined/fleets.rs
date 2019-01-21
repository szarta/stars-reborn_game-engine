/*
 *  Copyright 2019 Brandon Arrendondo
 *
 *  Permission is hereby granted, free of charge, to any person obtaining a
 *  copy of this software and associated documentation files (the "Software"),
 *  to deal in the Software without restriction, including without limitation
 *  the rights to use, copy, modify, merge, publish, distribute, sublicense,
 *  and/or sell copies of the Software, and to permit persons to whom the
 *  Software is furnished to do so, subject to the following conditions:
 *
 *  The above copyright notice and this permission notice shall be included in
 *  all copies or substantial portions of the Software.
 *
 *  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 *  THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 *  FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 *  DEALINGS IN THE SOFTWARE.
 */
use game::objects::tech::TechnologyId;
use game::objects::fleet::ShipDesign;

/* 
 * Provided here are pre-defined ship archetypes.  Some notes:
 *  - these are meant to be cloned, thus const, their clones will be mutable
 *  - id will change as the design is cloned and given ownership to a player
 *  - engines of initial ships vary with tech capability (e.g. Fuel Mizer)
 *  - the names of ships for AI races vary with their race name
 */
pub const SPORE_CLOUD = ShipDesign {
    name: "Spore Cloud",
    id: 0,
    base_hull: TechnologyId::MiniColonyShip,
    slots: Some([
                Some(ShipSlot {
                    tid: TechnologyId::SettlersDelight,
                    amount: 1
                }),
                Some(ShipSlot {
                    tid: TechnologyId::ColonizationModule,
                    amount: 1
                }),
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None
            ])
};

pub const SANTA_MARIA = ShipDesign {
    name: "Santa Maria",
    id: 0,
    base_hull: TechnologyId::ColonyShip,
    slots: Some([
                Some(ShipSlot {
                    tid: TechnologyId::QuickJump5,
                    amount: 1
                }),
                Some(ShipSlot {
                    tid: TechnologyId::ColonizationModule,
                    amount: 1
                }),
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None
            ])
};


