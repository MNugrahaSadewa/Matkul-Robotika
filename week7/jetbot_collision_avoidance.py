# Copyright 1996-2021 Cyberbotics Ltd.
# Licensed under the Apache License, Version 2.0

"""jetbot_collision_avoidance controller."""

import torch
import torchvision.transforms as transforms
import torch.nn.functional as F
import torchvision
from torchvision.models import ResNet18_Weights
import PIL.Image
import os.path
import warnings

# Mengabaikan peringatan sementara
warnings.filterwarnings("ignore", category=UserWarning)
warnings.filterwarnings("ignore", category=FutureWarning)

from jetbot_python_control import JetBot

# Normalisasi untuk input gambar
mean = torch.Tensor([0.485, 0.456, 0.406])
std = torch.Tensor([0.229, 0.224, 0.225])
normalize = transforms.Normalize(mean, std)

# Path model
model_path = 'best_model_resnet18.pth'
device = torch.device('cpu')  # Menggunakan CPU, jika GPU tidak tersedia

def preprocessCameraImage(camera):
    global normalize
    data = camera.getImage()
    image = PIL.Image.frombytes('RGBA', (camera.getWidth(), camera.getHeight()), data, 'raw', 'BGRA').convert('RGB')
    image = transforms.functional.to_tensor(image).to(device)
    image = normalize(image)
    return image.unsqueeze(0)  # Menambahkan dimensi batch

# Mengecek apakah file model tersedia
if not os.path.isfile(model_path):
    print(f'Trained model "{model_path}" not found, please use the "jetbot_collect_data" controller to generate it.')
    exit()

# Buat instance Robot
robot = JetBot()
timestep = 5 * int(robot.getBasicTimeStep())

# Aktifkan kamera robot
robot.camera.enable(timestep)
robot.step(10 * timestep)

print('Load the trained model...')
model = torchvision.models.resnet18(weights=ResNet18_Weights.DEFAULT)  # Menggunakan bobot ResNet18 terlatih
model.fc = torch.nn.Linear(512, 2)

# Load model dengan weights_only
try:
    model.load_state_dict(torch.load(model_path, weights_only=True))
except TypeError:
    model.load_state_dict(torch.load(model_path))

model = model.to(device)
model.eval()

# Main loop
print('Start collision avoidance control...')
direction = ''
while robot.step(timestep) != -1:
    camera_value = preprocessCameraImage(robot.camera)
    with torch.no_grad():  # Disable grad untuk inference
        y = model(camera_value)
        y = F.softmax(y, dim=1)
    prob_blocked = float(y[0, 0])  # Ambil probabilitas "blocked"

    if prob_blocked < 0.5:
        # Deteksi jalur bebas, gerak maju
        if direction != 'forward':
            robot.forward(0.5)
            direction = 'forward'
    elif direction != 'left':
        # Terdeteksi rintangan, belok kiri
        robot.left(0.4)
        direction = 'left'
