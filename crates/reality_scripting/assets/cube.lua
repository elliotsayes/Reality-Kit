local app = ...

local move_system_params = {
    Commands,
    {
        cube.CubeMarker.ref,
        bevy_transform.components.transform.Transform.mut,
    },
    bevy_time.time["Time<()>"].ref
}
function move_system(commands, query, time_res)
    local e = time_res:elapsed_secs()
    local d = time_res:delta_secs()
    for _, transform in query:iter() do
        print(e, d, transform.translation)
        transform.translation.x = transform.translation.x + 0.01 -- * d
        transform.translation.y = transform.translation.y + 0.005 -- * d
        transform.translation.z = transform.translation.z + 0.0002 -- * d
    end
end

app:register_system(move_system, move_system_params)
