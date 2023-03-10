-- -- -- -- -- --
-- gui/topbar  --
-- -- -- -- -- --

function new_topbar(params)
    local score = params.score
    local mode = params.mode

    local t = {}

    --

    function t.draw()
        -- TODO
        --    rectfill(
        --        a.camera_x,
        --        a.camera_y,
        --        a.camera_x + u.screen_px,
        --        a.camera_y + a.topbar_h - 1,
        --        u.colors.black
        --    )
        --
        --    local mode_label = mode.label()
        --    if mode_label then
        --        local text_y = a.camera_y + 4
        --        local progress_w = u.measure_text_width(mode_label)
        --        local progress_remaining_w = (mode.percentage_left() / 100) * progress_w
        --        local progress_x = a.camera_x + u.screen_px - progress_w - 1
        --        local progress_y = text_y + u.text_height_px + 2
        --        local progress_h = 1
        --        print(
        --            mode_label,
        --            progress_x,
        --            text_y,
        --            u.colors.light_grey
        --        )
        --        if progress_remaining_w > 0 then
        --            line(
        --                progress_x + progress_w - progress_remaining_w,
        --                progress_y,
        --                progress_x + progress_w - 1,
        --                progress_y + progress_h - 1,
        --                mode.progress_color()
        --            )
        --        end
        --    end
        --
        --    print(
        --        "score " .. tostr(score.value()),
        --        a.camera_x + 1,
        --        a.camera_y + 4,
        --        u.colors.light_grey
        --    )
    end

    --

    return t
end
