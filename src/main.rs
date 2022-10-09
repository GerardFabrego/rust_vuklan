use vulkano::{
    instance::{Instance, InstanceCreateInfo},
    device::physical::PhysicalDevice,
    device::{Device, DeviceCreateInfo, Features, QueueCreateInfo},
    buffer::{BufferUsage, CpuAccessibleBuffer}
};


fn main() {
    let instance = Instance::new(InstanceCreateInfo::default()).expect("failed to create instance");
    let physical = PhysicalDevice::enumerate(&instance).next().expect("no device available");

    let queue_family = physical.queue_families()
    .find(|&q| q.supports_graphics())
    .expect("couldn't find a graphical queue family");

    let (device, mut queues) = Device::new(
        physical,
        DeviceCreateInfo {
            // here we pass the desired queue families that we want to use
            queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
            ..Default::default()
        },
    )
    .expect("failed to create device");

    let queue = queues.next().unwrap();


    // Create buffer from iterator
    let iter = (0..128).map(|_| 5u8);
    let buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, iter).unwrap();


    // Access and modify buffer data
    let mut content = buffer.write().unwrap();
    
    content[12] = 83;
    content[7] = 3;

}
