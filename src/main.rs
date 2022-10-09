use vulkano::{
    instance::{Instance, InstanceCreateInfo},
    device::physical::PhysicalDevice,
    device::{Device, DeviceCreateInfo, Features, QueueCreateInfo},
    buffer::{BufferUsage, CpuAccessibleBuffer},
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferInfo},
    sync:: {self, GpuFuture}
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
    let source_content: Vec<i32> = (0..64).collect();
    let source = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, source_content)
        .expect("failed to create buffer");

    let destination_content: Vec<i32> = (0..64).map(|_| 0).collect();
    let destination = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, destination_content)
        .expect("failed to create buffer");


    // Build command buffer
    let mut builder = AutoCommandBufferBuilder::primary(
        device.clone(),
        queue.family(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    builder.copy_buffer(CopyBufferInfo::buffers(source.clone(), destination.clone())).unwrap();

    let command_buffer = builder.build().unwrap();


    // Send command buffer to GPU and execute it
    let future = sync::now(device.clone())
    .then_execute(queue.clone(), command_buffer)
    .unwrap()
    .then_signal_fence_and_flush() // same as signal fence, and then flush
    .unwrap();

    future.wait(None).unwrap();

    let src_content = source.read().unwrap();
    let destination_content = destination.read().unwrap();
    assert_eq!(&*src_content, &*destination_content);
}
