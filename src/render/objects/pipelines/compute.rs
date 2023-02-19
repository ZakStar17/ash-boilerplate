use std::{ffi::CString, ptr};

use ash::vk;

use crate::render::{objects::DescriptorSets, shaders::ComputeShaders};

pub struct ComputePipelines {
  pub layout: vk::PipelineLayout,
  pub instance: vk::Pipeline,
}

impl ComputePipelines {
  pub fn create(device: &ash::Device, descriptor_sets: &DescriptorSets) -> Self {
    let mut shaders = ComputeShaders::load(device);
    let main_function_name = CString::new("main").unwrap(); // the beginning function name in shader code.

    let stage = vk::PipelineShaderStageCreateInfo {
      s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
      p_next: ptr::null(),
      flags: vk::PipelineShaderStageCreateFlags::empty(),
      module: shaders.instance,
      p_name: main_function_name.as_ptr(),
      p_specialization_info: ptr::null(),
      stage: vk::ShaderStageFlags::COMPUTE,
    };

    // TODO:
    // add push constants

    let set_layouts = [descriptor_sets.layouts.instance_compute.layout];
    let layout_create_info = vk::PipelineLayoutCreateInfo {
      s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
      p_next: ptr::null(),
      flags: vk::PipelineLayoutCreateFlags::empty(),
      set_layout_count: set_layouts.len() as u32,
      p_set_layouts: set_layouts.as_ptr(),
      push_constant_range_count: 0,
      p_push_constant_ranges: ptr::null(),
    };

    let layout = unsafe {
      device
        .create_pipeline_layout(&layout_create_info, None)
        .expect("Failed to create pipeline layout!")
    };

    let create_infos = [vk::ComputePipelineCreateInfo {
      s_type: vk::StructureType::COMPUTE_PIPELINE_CREATE_INFO,
      p_next: ptr::null(),
      stage,
      flags: vk::PipelineCreateFlags::empty(),
      layout,
      base_pipeline_handle: vk::Pipeline::null(),
      base_pipeline_index: -1,
    }];

    let pipelines = unsafe {
      device
        .create_compute_pipelines(vk::PipelineCache::null(), &create_infos, None)
        .expect("Failed to create compute pipelines")
    };
    let mut pipelines_iter = pipelines.into_iter();

    unsafe {
      shaders.destroy_self(device);
    }

    Self {
      layout,
      instance: pipelines_iter.next().unwrap(),
    }
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    device.destroy_pipeline(self.instance, None);
    device.destroy_pipeline_layout(self.layout, None);
  }
}
