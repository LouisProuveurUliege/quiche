// Copyright (C) 2019, Cloudflare, Inc.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//
//     * Redistributions in binary form must reproduce the above copyright
//       notice, this list of conditions and the following disclaimer in the
//       documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
// IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! NOCC Congestion Control
//!
//! Disable CC for Deep Space usage. only flow control

use std::time::Instant;

use crate::recovery::rtt::RttStats;
use crate::recovery::congestion::{Acked};
use crate::recovery::Sent;

use super::Congestion;
use super::CongestionControlOps;

pub(crate) static NOCC: CongestionControlOps = CongestionControlOps {
    on_init,
    on_packet_sent,
    on_packets_acked,
    congestion_event,
    checkpoint,
    rollback,
    debug_fmt,
    #[cfg(feature="qlog")]
    state_str,
};
#[allow(unused_variables)]
fn on_init(_r: &mut Congestion) {}
#[allow(unused_variables)]
fn on_packet_sent(
    r: &mut Congestion, sent_bytes: usize, bytes_in_flight: usize, now: Instant,
) {}
#[allow(unused_variables)]
fn on_packets_acked(
    r: &mut Congestion, bytes_in_flight: usize, packets: &mut Vec<Acked>,
    now: Instant, rtt_stats: &RttStats,
) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn on_packet_acked(
    r: &mut Congestion, bytes_in_flight: usize, packet: &Acked, now: Instant,
    rtt_stats: &RttStats,
) {}
#[allow(unused_variables)]
fn congestion_event(
    r: &mut Congestion, bytes_in_flight: usize, _lost_bytes: usize,
    largest_lost_pkt: &Sent, now: Instant,
) {}
#[allow(unused_variables)]
fn checkpoint(r: &mut Congestion) {}
#[allow(unused_variables)]
fn rollback(r: &mut Congestion) -> bool { true }

#[cfg(feature = "qlog")]
#[allow(unused_variables)]
fn state_str(r: &Congestion, now: Instant) -> &'static str { "noop" }
#[allow(unused_variables)]
fn debug_fmt(r: &Congestion, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
        f,
        "nocc="
    )
}
