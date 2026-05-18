### “Experiment 2.1: Original code, and how it run”

![img.png](img.png)

Cara Menjalankan Aplikasi:
Untuk mengoperasikan aplikasi obrolan ini, langkah pertama yang harus dilakukan adalah menjalankan program peladen (server) melalui terminal utama dengan mengeksekusi perintah cargo run --bin server. Setelah peladen aktif mendengarkan koneksi jaringan pada port yang ditentukan, buka tiga buah jendela terminal baru secara terpisah untuk bertindak sebagai klien mandiri. Pada masing-masing terminal klien tersebut, jalankan perintah cargo run --bin client untuk menginisiasi proses jabat tangan protokol WebSocket menuju alamat peladen.

Analisis Alur Komunikasi Saat Mengetik Pesan:
Ketika seorang pengguna mengetikkan pesan teks di salah satu terminal klien dan menekan tombol enter, makro tokio::select! pada sisi klien akan langsung menangkap baris input tersebut melalui saluran standard input asinkronus dan mengirimkannya sebagai pesan teks WebSocket menuju peladen. Saat pesan tersebut bersandar di peladen, fungsi handle_connection akan mengekstrak isi teks dan menyalurkannya ke dalam saluran broadcast channel utama milik peladen. Saluran siaran ini secara otomatis akan menggandakan dan mendistribusikan pesan tersebut ke seluruh instansi receiver aktif yang terikat pada masing-masing koneksi klien lain. Akibatnya, setiap teks yang diketik oleh satu klien akan langsung muncul secara seketika (real-time) di layar terminal milik semua klien lainnya yang sedang terhubung, menunjukkan keberhasilan implementasi penanganan konkurensi asinkronus berbasis protokol WebSocket.