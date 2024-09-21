ffmpeg -i out.mp4 -framerate 20 'frames/frame-%03d.png'
ffmpeg -framerate 20 -pattern_type glob -i 'out/*.png' -c:v libx264 -pix_fmt yuv420p out.mp4
