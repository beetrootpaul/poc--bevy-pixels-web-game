-- -- -- -- -- -- --
-- gameplay/trail --
-- -- -- -- -- -- --

function new_trail(params)
    local origin = params.origin
    local color = params.color

    -- TODO
    --local frames_between_particles = 4
    --local frame_counter = frames_between_particles

    local particles = {}

    local t = {}

    --

    function t.update()
        -- TODO
        --for particle in all(particles) do
        --    particle.age()
        --end
        --for i = 1, #particles do
        --    if particles[i] then
        --        particles[i].age()
        --        if particles[i].should_disappear() then
        --            deli(particles, i)
        --            particles[i] = particles[#particles]
        --            particles[#particles] = nil
        --        end
        --    end
        --end
    end

    --

    function t.draw()
        -- TODO
        --for particle in all(particles) do
        --    particle.draw()
        --end
    end

    --

    return t
end