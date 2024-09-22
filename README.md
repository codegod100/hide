
ffmpeg -framerate 20 -pattern_type glob -i 'out/*.png' -c:v libx264 -b:v 10000k -threads 16 -pix_fmt yuv420p out.mp4 && ffmpeg -i out.mp4 -framerate 20 'frames/qrcode-%03d.png'

ffmpeg -i download.mp4 -framerate 20 'frames/qrcode-%03d.png'
