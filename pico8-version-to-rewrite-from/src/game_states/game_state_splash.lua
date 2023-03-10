-- -- -- -- -- -- -- -- -- -- -- --
-- game_states/game_state_splash --
-- -- -- -- -- -- -- -- -- -- -- --

function new_game_state_splash()
    -- TODO
    --local sash = new_sash({
    --    duration = 10 * a.music_beat_frames,
    --    expand = false,
    --    draw_text = function(sash_center_x, sash_center_y)
    --        local title = "avoid your past"
    --        local title_w = u.measure_text_width(title)
    --        local author = "by @beetrootpaul"
    --        local author_w = u.measure_text_width(author)
    --        u.print_with_outline(
    --            title,
    --            sash_center_x - title_w / 2,
    --            sash_center_y - u.text_height_px - 3,
    --            u.colors.pink,
    --            u.colors.black
    --        )
    --        print(
    --            author,
    --            sash_center_x - author_w / 2,
    --            sash_center_y + 2,
    --            u.colors.white
    --        )
    --    end,
    --})

    audio.play_music()
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
        --if btnp(u.buttons.l) or btnp(u.buttons.r) or btnp(u.buttons.u) or btnp(u.buttons.d) then
        --    sash.collapse()
        --end

        -- TODO
        --sash.advance_1_frame()

        return gs
    end

    --

    function gs.draw()
        -- TODO
        --rectfill(
        --    a.camera_x,
        --    a.camera_y,
        --    a.camera_x + u.screen_px - 1,
        --    a.camera_y + u.screen_px - 1,
        --    a.bg_color_mode_normal
        --)
        --sash.draw()
    end

    --

    return gs
end