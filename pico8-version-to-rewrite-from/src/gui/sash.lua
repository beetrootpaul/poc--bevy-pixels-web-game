-- -- -- -- --
-- gui/sash --
-- -- -- -- --

function new_sash(params)
    local ttl_max = params.duration
    local should_expand = params.expand
    local draw_text = params.draw_text

    -- TODO
    --local ttl_expansion_start = should_expand and (ttl_max - a.music_beat_frames) or ttl_max
    --local ttl_expansion_end = should_expand and (ttl_expansion_start - a.music_beat_frames / 4) or ttl_max
    --local ttl_collapse_start = a.music_beat_frames / 4
    --local ttl = ttl_max

    -- TODO
    --local center_x = a.camera_x + u.screen_px / 2
    --local center_y = a.camera_y + u.screen_px / 2
    --local h_max = 30

    local s = {}

    --

    -- TODO
    --function s.has_collapsed()
    --    return ttl <= 0
    --end

    --

    -- TODO
    --function s.has_expanded()
    --    return ttl <= ttl_expansion_end
    --end

    --

    -- TODO
    --function s.collapse()
    --    ttl = ttl_collapse_start
    --end

    --

    -- TODO
    --function s.advance_1_frame()
    --    ttl = ttl - 1
    --end

    --

    -- TODO
    --function s.draw()
    --    local h
    --    if ttl > ttl_expansion_start then
    --        h = 0
    --    elseif ttl > ttl_expansion_end then
    --        h = h_max * (ttl_expansion_start - ttl) / (ttl_expansion_start - ttl_expansion_end)
    --    elseif ttl > ttl_collapse_start then
    --        h = h_max
    --    else/
    --        h = h_max * ttl / ttl_collapse_start
    --    end
    --
    --    if h > 0 then
    --        rectfill(
    --            0, center_y - h / 2,
    --            u.screen_px - 1, center_y + h / 2 - 1,
    --            u.colors.dark_green
    --        )
    --    end
    --
    --    if h >= h_max then
    --        draw_text(center_x, center_y)
    --    end
    --end

    --

    return s
end