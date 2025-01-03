# Laporan UAS Robotika

## **Deskripsi Proyek**
📖 Laporan ini merupakan dokumentasi dari simulasi dan implementasi robot lengan 7 Degree of Freedom (DOF) menggunakan Robot Operating System (ROS). Kami menyelesaikan berbagai tahapan simulasi, termasuk implementasi komunikasi antar node, pengendalian sendi, dan simulasi gerakan robot dalam lingkungan virtual.

---
## **Link Video Demonstrasi**
📺 Silakan lihat video demonstrasi proyek ini melalui playlist berikut:
[YouTube Playlist - Laporan UAS Robotika](https://www.youtube.com/playlist?list=PL8B8pFsoZrsO8xDsnjS6CBqwJK5w3P-U6)
---
## **Chapter yang Diselesaikan**

### **Pengerjaan**
1. **Menjalankan Antarmuka MoveIt!:**
   - **Command:** `roslaunch seven_dof_arm_config demo.launch` 🚀
   - **Hasil:** Antarmuka MoveIt! terbuka dan berfungsi dengan baik. ✅

2. **Simulasi di Gazebo:**
   - **Command:** `roslaunch seven_dof_arm_gazebo seven_dof_arm_bringup_moveit.launch` 🌐
   - **Hasil:** Simulasi di Gazebo menunjukkan robot bergerak sesuai perintah. 🦾

3. **Menambahkan Objek Kolisi:**
   - **Command:** `rosrun seven_dof_arm_test add_collision_object` 🔧
   - **Hasil:** Robot mendeteksi dan menghindari objek kolisi dengan akurat. 🎯

4. **Melakukan Tugas Pick and Place:**
   - **Command:** `rosrun seven_dof_arm_test pick_place` 📦
   - **Hasil:** Robot berhasil memindahkan objek tanpa kendala. 🤖

5. **Kontrol Sendi Robot di CoppeliaSim:**
   - **Command untuk menggerakkan sendi:** `rostopic pub` 🛠️
   - **Command untuk verifikasi status:** `rostopic echo` 🖥️
   - **Hasil:** Sendi robot bergerak sesuai perintah, dan status diverifikasi dengan tepat. 🔍

6. **Implementasi Komunikasi Antar Node:**
   - **Publisher dan Subscriber:**
     - **Publisher:** `rosrun mastering_ros_demo_pkg demo_topic_publisher` 📡
     - **Subscriber:** `rosrun mastering_ros_demo_pkg demo_topic_subscriber` 🎧
     - **Hasil:** Data berhasil diterbitkan dan diterima tanpa kendala. 🔗
   - **Service untuk Komunikasi Sinkron:**
     - **Server:** `rosrun mastering_ros_demo_pkg demo_service_server` 🖥️
     - **Client:** `rosrun mastering_ros_demo_pkg demo_service_client` 💡
     - **Hasil:** Respons server diterima dengan baik oleh client. ✔️

---

## **Hasil Simulasi yang Dicapai**
1. **Antarmuka MoveIt!:** Berhasil digunakan untuk memvalidasi gerakan robot. 🤩
2. **Integrasi Gazebo:** Simulasi berjalan lancar dengan gerakan presisi. 🌟
3. **Pengujian Objek Kolisi:** Robot mendeteksi dan menghindari objek kolisi dengan akurat. 🎯
4. **Tugas Pick and Place:** Robot memindahkan objek tanpa tabrakan. 🦾
5. **Komunikasi Antar Node:** Publisher-Subscriber dan Service berfungsi dengan optimal. 🔗
6. **Kontrol di CoppeliaSim:** Sendi robot dikontrol dan statusnya diverifikasi dengan presisi. 📋

---

## **Tantangan yang Dihadapi**
1. **Konfigurasi Dependensi:** Masalah pada beberapa library ROS memerlukan debugging tambahan. 🛠️
2. **Sinkronisasi Gerakan:** Integrasi MoveIt! dan Gazebo memerlukan pengaturan ulang untuk sinkronisasi yang sempurna. ⚙️
3. **Komunikasi Node:** Implementasi Service membutuhkan penyesuaian pada logika respons server. 🧩

---

## **Insight yang Didapatkan**
1. **Simulasi adalah Langkah Kunci:** Validasi sistem secara virtual sebelum implementasi fisik mengurangi risiko kesalahan. ✅
2. **Fleksibilitas ROS:** Publisher, Subscriber, dan Service memungkinkan pengembangan sistem komunikasi yang scalable. 📡
3. **Kolaborasi Alat:** Integrasi MoveIt!, Gazebo, dan CoppeliaSim memberikan gambaran holistik tentang pengendalian robot. 🤝

---

## **Kesimpulan**
🎓 Proyek ini menunjukkan kemampuan kami dalam:
1. Mengimplementasikan ROS untuk simulasi robot lengan 7 DOF. 🦾
2. Menggunakan berbagai tools untuk validasi sistem. ⚙️
3. Mengembangkan komunikasi node yang mendukung pengendalian robot secara kompleks. 🔗





