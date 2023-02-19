use std::ptr;

use ash::vk;

pub struct SizedLayout {
  pub layout: vk::DescriptorSetLayout,
  pub descriptor_count: usize,
}

pub struct DescriptorSetLayouts {
  pub instance_compute: SizedLayout,
}

impl DescriptorSetLayouts {
  pub fn new(device: &ash::Device) -> Self {
    let bindings = [
      vk::DescriptorSetLayoutBinding {
        binding: 0,
        descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
        descriptor_count: 1,
        stage_flags: vk::ShaderStageFlags::COMPUTE,
        p_immutable_samplers: ptr::null(),
      },
      vk::DescriptorSetLayoutBinding {
        binding: 1,
        descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
        descriptor_count: 1,
        stage_flags: vk::ShaderStageFlags::COMPUTE,
        p_immutable_samplers: ptr::null(),
      },
    ];
    let descriptor_count = bindings
      .iter()
      .map(|binding| binding.descriptor_count)
      .sum::<u32>() as usize;

    let create_info = vk::DescriptorSetLayoutCreateInfo {
      s_type: vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
      p_next: ptr::null(),
      flags: vk::DescriptorSetLayoutCreateFlags::empty(),
      binding_count: bindings.len() as u32,
      p_bindings: bindings.as_ptr(),
    };

    let layout = unsafe {
      device
        .create_descriptor_set_layout(&create_info, None)
        .expect("Failed to create descriptor set layout")
    };

    Self {
      instance_compute: SizedLayout {
        layout,
        descriptor_count,
      },
    }
  }

  pub unsafe fn destroy_self(&mut self, device: &ash::Device) {
    device.destroy_descriptor_set_layout(self.instance_compute.layout, None);
  }
}
