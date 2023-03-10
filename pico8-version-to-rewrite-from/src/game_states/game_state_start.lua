-- -- -- -- -- -- -- -- -- -- -- --
-- game_states/game_state_start  --
-- -- -- -- -- -- -- -- -- -- -- --

function new_game_state_start()
    -- TODO
    --local score = new_score()
    --local mode = new_mode()
    --local topbar = new_topbar {
    --    score = score,
    --    mode = mode,
    --}
    --local player = new_player()
    --local level = new_level {
    --    mode = mode,
    --    player = player,
    --}

    -- TODO
    --audio.enable_music_layers { false, false, false }

    -- TODO
    --level.spawn_items()

    local gs = {}

    --

    function gs.update()
        -- TODO
        --local has_started = false
        --if btnp(u.buttons.l) then
        --    player.direct_left()
        --    has_started = true
        --elseif btnp(u.buttons.r) then
        --    player.direct_right()
        --    has_started = true
        --elseif btnp(u.buttons.u) then
        --    player.direct_up()
        --    has_started = true
        --elseif btnp(u.buttons.d) then
        --    player.direct_down()
        --    has_started = true
        --end

        -- TODO
        --level.animate()

        -- TODO
        --if has_started then
        --    return new_game_state_gameplay {
        --        mode = mode,
        --        topbar = topbar,
        --        score = score,
        --        level = level,
        --        player = player,
        --    }
        --end

        return gs
    end

    --

    function gs.draw()
        -- TODO
        --level.draw_bg()
        --level.draw_items()
        --player.draw()
        --topbar.draw()

        -- TODO
        --local margin = 6
        --local prompt1 = "press an arrow"
        --local prompt2 = "to choose direction"
        --local prompt1_w = u.measure_text_width(prompt1)
        --local prompt2_w = u.measure_text_width(prompt2)
        --u.print_with_outline(prompt1, player.xc() - prompt1_w / 2, player.y1() - margin - 26, u.colors.violet_grey, u.colors.dark_blue)
        --u.print_with_outline(prompt2, player.xc() - prompt2_w / 2, player.y1() - margin - 17, u.colors.violet_grey, u.colors.dark_blue)
        --local time_dependent_boolean = u.boolean_changing_every_nth_second(a.music_beat_frames / a.fps)
        --local glyph_color = time_dependent_boolean and u.colors.violet_grey or u.colors.blue
        --u.print_with_outline("⬅️", player.x1() - margin - 8, player.yc() - 2, glyph_color, u.colors.dark_blue)
        --u.print_with_outline("➡️", player.x2() + margin + 2, player.yc() - 2, glyph_color, u.colors.dark_blue)
        --u.print_with_outline("⬆️", player.xc() - 3, player.y1() - margin - 6, glyph_color, u.colors.dark_blue)
        --u.print_with_outline("⬇️", player.xc() - 3, player.y2() + margin + 2, glyph_color, u.colors.dark_blue)
    end

    --

    return gs
end