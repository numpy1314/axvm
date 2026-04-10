// Copyright 2025 The Axvisor Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_std]
#![feature(new_range_api)]
// #![feature(concat_idents)]
// #![feature(naked_functions)]
// #![feature(const_trait_impl)]

//! This crate provides a minimal VM monitor (VMM) for running guest VMs.
//!
//! This crate contains:
//! - [`AxVM`]: The main structure representing a VM.

extern crate alloc;
#[macro_use]
extern crate log;

mod hal;
mod vcpu;
mod vm;

pub mod config;

pub use vm::{AxVCpuRef, AxVM, AxVMRef, VMMemoryRegion, VMStatus};

/// The architecture-independent per-CPU type.
pub type AxVMPerCpu = axvcpu::AxPerCpu<vcpu::AxVMArchPerCpuImpl>;

/// The interfaces which the underlying software (kernel or hypervisor) must implement.
pub trait AxVMHal: Sized {
    /// The low-level **OS-dependent** helpers that must be provided for physical address management.
    type PagingHandler: page_table_multiarch::PagingHandler;

    /// Converts a virtual address to the corresponding physical address.
    fn virt_to_phys(vaddr: axaddrspace::HostVirtAddr) -> axaddrspace::HostPhysAddr;

    /// Current time in nanoseconds.
    fn current_time_nanos() -> u64;

    /// Current VM ID.
    fn current_vm_id() -> usize;

    /// Current Virtual CPU ID.
    fn current_vcpu_id() -> usize;

    /// Current Physical CPU ID.
    fn current_pcpu_id() -> usize;

    /// Get the Physical CPU ID where the specified VCPU of the current VM resides.
    ///
    /// Returns an error if the VCPU is not found.
    fn vcpu_resides_on(vm_id: usize, vcpu_id: usize) -> axerrno::AxResult<usize>;

    /// Inject an IRQ to the specified VCPU.
    ///
    /// This method should find the physical CPU where the specified VCPU resides and inject the IRQ
    /// to it on that physical CPU with [`axvcpu::AxVCpu::inject_interrupt`].
    ///
    /// Returns an error if the VCPU is not found.
    fn inject_irq_to_vcpu(vm_id: usize, vcpu_id: usize, irq: usize) -> axerrno::AxResult;
}

/// Whether the hardware has virtualization support.
pub fn has_hardware_support() -> bool {
    vcpu::has_hardware_support()
}
