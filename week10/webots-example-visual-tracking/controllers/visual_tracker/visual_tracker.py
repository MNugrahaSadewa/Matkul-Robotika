"""This controller makes the robot follow the red ball."""

import cv2
import numpy as np
from controller import Robot

# P value for P controller
P_COEFFICIENT = 0.1

# Threshold untuk mendeteksi bola (sesuaikan jika perlu)
AREA_THRESHOLD = 50 

# Initialize the robot
robot = Robot()
timestep = int(robot.getBasicTimeStep())

# Initialize camera
camera = robot.getDevice('camera')
camera.enable(timestep)

# Initialize motors
motor_left = robot.getDevice('left wheel motor')
motor_right = robot.getDevice('right wheel motor')
motor_left.setPosition(float('inf'))
motor_right.setPosition(float('inf'))


# Main control loop
while robot.step(timestep) != -1:
    img = np.frombuffer(camera.getImage(), dtype=np.uint8).reshape((camera.getHeight(), camera.getWidth(), 4))

    # Segment the image by color in HSV color space
    hsv = cv2.cvtColor(img, cv2.COLOR_RGB2HSV) # Ubah nama variabel untuk kejelasan
    mask = cv2.inRange(hsv, np.array([50, 150, 0]), np.array([200, 230, 255]))



    # Find contours
    contours, _ = cv2.findContours(mask, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)

    # Find the largest contour above the area threshold
    largest_contour = None
    if contours:
        largest_contour = max(contours, key=cv2.contourArea)

        if cv2.contourArea(largest_contour) < AREA_THRESHOLD:
             largest_contour = None # Abaikan kontur yang terlalu kecil


    # Jika bola terdeteksi, hitung error dan atur kecepatan motor
    if largest_contour is not None:
        M = cv2.moments(largest_contour)
        if M["m00"] != 0: # Hindari pembagian dengan nol
            center_x = int(M["m10"] / M["m00"])
            error = camera.getWidth() / 2 - center_x
            motor_left.setVelocity(-error * P_COEFFICIENT)
            motor_right.setVelocity(error * P_COEFFICIENT)

    else: # Bola tidak terdeteksi, hentikan robot
        print("Bola terdeteksi!")
        motor_left.setVelocity(0)
        motor_right.setVelocity(0)