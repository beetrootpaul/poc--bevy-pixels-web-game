-- -- -- -- -- -- -- -- -- -- --
-- gameplay/animated_sprite   --
-- -- -- -- -- -- -- -- -- -- --

function new_animated_sprite(params)
    local first_sprite = params.first_sprite
    local number_of_sprites = params.number_of_sprites
    local frames_per_sprite = params.frames_per_sprite

    -- TODO
    --local frame_counter = 0
    --local loop_length_frames = frames_per_sprite * number_of_sprites

    local as = {}

    --

    function as.advance_1_frame()
        -- TODO
        --frame_counter = (frame_counter + 1) % loop_length_frames
    end

    --

    function as.current_sprite()
        -- TODO
        --local sprite_index = flr(frame_counter / frames_per_sprite)
        --return first_sprite + sprite_index
    end

    --

    return as
end