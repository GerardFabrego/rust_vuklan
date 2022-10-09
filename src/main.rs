use vulkano::{
    instance::{Instance, InstanceCreateInfo},
    device::physical::PhysicalDevice,
    device::{Device, DeviceCreateInfo, Features, QueueCreateInfo}
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
}
