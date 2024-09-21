ffmpeg -framerate 100 -pattern_type glob -i 'out/*.png' -c:v libx264 -pix_fmt yuv420p out.mp4 && ffmpeg -i out.mp4 -framerate 100 'frames/qrcode-%03d.png'
