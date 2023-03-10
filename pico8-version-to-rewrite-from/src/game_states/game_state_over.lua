-- -- -- -- -- -- -- -- -- -- -- --
-- game_states/game_state_over   --
-- -- -- -- -- -- -- -- -- -- -- --

function new_game_state_over(params)
    local score = params.score
    local level = params.level
    local player = params.player

    -- TODO
    --local sash = new_sash({
    --    duration = 10 * a.music_beat_frames,
    --    expand = true,
    --    draw_text = function(sash_center_x, sash_center_y)
    --        local heading = "your score"
    --        local heading_w = u.measure_text_width(heading)
    --        local final_score = tostr(score.value())
    --        local final_score_w = u.measure_text_width(final_score)
    --        print(
    --            heading,
    --            sash_center_x - heading_w / 2,
    --            sash_center_y - u.text_height_px - 3,
    --            u.colors.white
    --        )
    --        u.print_with_outline(
    --            final_score,
    --            sash_center_x - final_score_w / 2,
    --            sash_center_y + 2,
    --            u.colors.pink,
    --            u.colors.black
    --        )
    --    end,
    --})

    -- TODO
    --audio.enable_music_layers { false, false, false }

    local gs = {}

    --

    function gs.update()
        -- TODO
        --if sash.has_collapsed() then
        --    return new_game_state_start()
        --end

        -- TODO
        --if sash.has_expanded() then
        --    if btnp(u.buttons.l) or btnp(u.buttons.r) or btnp(u.buttons.u) or btnp(u.buttons.d) then
        --        sash.collapse()
        --    end
        --end

        -- TODO
        --sash.advance_1_frame()

        return gs
    end

    --

    function gs.draw()
        -- TODO
        --level.draw_bg()
        --level.draw_items()
        --player.draw()

        -- TODO
        --sash.draw()
    end

    --

    return gs
end
