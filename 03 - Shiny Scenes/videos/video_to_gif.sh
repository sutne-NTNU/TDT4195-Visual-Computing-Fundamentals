ffmpeg -i doors.mp4 -filter_complex "[0:v] fps=24,scale=480:-1,split [a][b];[a] palettegen=stats_mode=single [p];[b][p] paletteuse=new=1" doors.gif
ffmpeg -i flyby.mp4 -filter_complex "[0:v] fps=24,scale=480:-1,split [a][b];[a] palettegen=stats_mode=single [p];[b][p] paletteuse=new=1" flyby.gif
ffmpeg -i flying.mp4 -filter_complex "[0:v] fps=24,scale=480:-1,split [a][b];[a] palettegen=stats_mode=single [p];[b][p] paletteuse=new=1" flying.gif
ffmpeg -i landing.mp4 -filter_complex "[0:v] fps=24,scale=480:-1,split [a][b];[a] palettegen=stats_mode=single [p];[b][p] paletteuse=new=1" landing.gif
ffmpeg -i takeoff.mp4 -filter_complex "[0:v] fps=24,scale=480:-1,split [a][b];[a] palettegen=stats_mode=single [p];[b][p] paletteuse=new=1" takeoff.gif