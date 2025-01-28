local app = ...

local i = 0.0;

function my_system(query)
    return
    --[[i = i + 1.0
    local last = nil
    for transform in query:iter() do
        last = transform.translation
        transform.translation.x = math.sin(i)
    end]]
end

local ep = 0.0001

function my_system2(commands, query, time_res)
    local t = time_res:elapsed_secs()
    print(t)
    for transform in query:iter() do
        transform.translation.x = transform.translation.x + 0.01
        transform.translation.y = transform.translation.y + 0.005
        transform.translation.z = transform.translation.z + 0.0002
    end
end

app:register_system(my_system2,
{
  Commands,
  {
    bevy_transform.components.transform.Transform.mut,
    cube.CubeMarker.mut,
  },
  bevy_time.time["Time<()>"].ref
})