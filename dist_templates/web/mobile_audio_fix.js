(function (w) {
    w.__mobile_audio_fix__ = {
        resume_audio_on_interactions_with,
    };

    // Events list taken from https://developer.chrome.com/blog/web-audio-autoplay/#moving-forward
    const userInputEventNames = [
        'click',
        'contextmenu',
        'auxclick',
        'dblclick',
        'mousedown',
        'mouseup',
        'pointerup',
        'touchend',
        'keydown',
        'keyup',
    ];

    const audioContextList = [];

    // AudioContext proxy taken from https://developer.chrome.com/blog/web-audio-autoplay/#moving-forward
    self.AudioContext = new Proxy(self.AudioContext, {
        construct(target, args) {
            const result = new target(...args);
            audioContextList.push(result);
            return result;
        },
    });

    function resume_audio_on_interactions_with(element) {
        userInputEventNames.forEach(eventName => {
            element.addEventListener(eventName, resumeAudioContexts);
        });
    }

    function resumeAudioContexts() {
        audioContextList.forEach(context => {
            if (context.state !== 'running') {
                context.resume();
            }
        });
    }
})(window);