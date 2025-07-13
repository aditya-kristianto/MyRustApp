# Implementasi Microservices Architecture

- Status: proposed
- Deciders: Alfa, Anggoro, Edy Salim
- Date: 2023-06-24
- Tags: miroservices, architectures

Technical Story: -

## Context and Problem Statement

Monolith Architectures yang sebelumnya sudah di implementasi oleh tim IT sekarang saat ini memiliki beberapa kekurangan, diantaranya code akan lebih kompleks dan besar sehingga akan menjadi sulit untuk dipelihara seiring dengan berjalannya waktu, itulah salah satu alasan mengapa perlu merubah konsep arsitektur menjadi arsitektur microservice. Konsep arsitektur monolit juga selalu membutuhkan infrastruktur perangkat yang lebih tinggi untuk melakukan komputasi ketika code aplikasi semakin kompleks dan berat yang akan menjadikan kendala bagi tim infrastruktur juga dalam pemenuhan kebutuhan perangkat keras. Sedangkan, kebutuhan untuk melakukan peningkatkan kebutuhan perangkat keras (scaling up) itu lebih mahal dan lambat jika di bandingkan dengan “horizontal scaling”.

## Decision Drivers

- 
- [driver 1, e.g., a force, facing concern, …]
- [driver 2, e.g., a force, facing concern, …]
- … <!-- numbers of drivers can vary -->

## Considered Options

- Microservices Architecture

## Decision Outcome

Kekurangan dari arsitektur monolit itulah yang sebagai alasan utama kita perlu merubah ke konsep topologi arsitektur yang terbaru. Arsitektur microservice adalah solusi yang dapat meningkatkan dan memperbaiki kompleksitas yang terjadi dalam arsitektur monolit.
Dalam arsitektur microservice, setiap service hanya akan terhubung ke satu database. Service tidak dapat terhubung ke database-database yang lain. Komunikasi untuk melakukan pembacaan atau update data akan menggunakan protokol API sebagai standar. Diagram gambar 1 di bawah ini merupakan contoh standar penerapan arsitektur microservice.

![Contoh yang baik dalam penerapan-microservices architecture](/l4b-static/assets/good-microservices.png "Contoh yang baik dalam penerapan-microservices architecture")

![Contoh yang tidak baik dalam penerapan microservices architecture](/l4b-static/assets/adr-workflow.png "Contoh yang tidak baik dalam penerapan microservices architecture")

![ADR workflow](/l4b-static/adr-workflow.png)

![ADR workflow](/l4b-static/good-microservices.png)

### Positive Consequences

-	Dapat menerapkan konsep praktik pengembangan dan integrasi berkelanjutan.
-	Dapat menerapkan konsep blue green deployment. 
-	Fleksibel untuk melakukan deploy di server publik atau server pribadi yang menggunakan platform berbeda seperti Windows Server atau Linux.
-	Isolasi komunikasi ke database.
-	Cepat dalam pembuatan dan pengembangan.
-	Mudah untuk melakukan perubahan dalam services dan tidak akan berdampak ke keseluruhan sistem.
-	Lebih mudah untuk melakukan memelihara dan merapikan code.
-	Mudah dan cepat meningkatkan kemampuan dengan horizontal scaling concept.
-	Dapat dibangun dan integrasi dengan lebih dari satu bahasa pemrograman. 
-	Dapat memitigasi terjadi gangguan pada sebuah service dan service yang lain dapat terus berfungsi.
-	Lebih mudah untuk melakukan pemeliharaan sistem.

### Negative Consequences

- [e.g., compromising quality attribute, follow-up decisions required, …]
- …

## Pros and Cons of the Options

### Microservices Architecture

#### Pros

-	Dapat menerapkan konsep praktik pengembangan dan integrasi berkelanjutan.
-	Dapat menerapkan konsep blue green deployment. 
-	Fleksibel untuk melakukan deploy di server publik atau server pribadi yang menggunakan platform berbeda seperti Windows Server atau Linux.
-	Isolasi komunikasi ke database.
-	Cepat dalam pembuatan dan pengembangan.
-	Mudah untuk melakukan perubahan dalam services dan tidak akan berdampak ke keseluruhan sistem.
-	Lebih mudah untuk melakukan memelihara dan merapikan code.
-	Mudah dan cepat meningkatkan kemampuan dengan horizontal scaling concept.
-	Dapat dibangun dan integrasi dengan lebih dari satu bahasa pemrograman. 
-	Dapat memitigasi terjadi gangguan pada sebuah service dan service yang lain dapat terus berfungsi.
-	Lebih mudah untuk melakukan pemeliharaan sistem.

#### Cons

-	Sistem akan menjadi lebih kompleks karena memungkinkan untuk di bangun dengan beberapa jenis bahasa pemrograman yang berbeda.
-	Koordinasi dan komunikasi antar service akan menjadi lebih kompleks.
-	Membutuhkan lebih banyak resources karena setiap service akan membutuhkan peran untuk menghubungkan ke masing-masing database dan memilihara service dan database jika terjadi gangguan.

### Monolith Architectures

#### Pros

-	Mudah dalam membangun sistem karena arsitektur monolith hanya menggunakan satu jenis bahasa pemrograman sehingga mudah dipelajari dan diadaptasi dalam tim.
-	Mudah untuk melakukan tes dan menelusuri karena semua tim dapat berkomunikasi dan melakukan pengecekan bersama dalam satu bahasa pemrograman tanpa perlu pengecekan di sisi komunikasi antar service.
-	Mudah dalam pengembangan karena komponennya lebih sederhana.

#### Cons

-	Susah untuk merubah fungsi dan pengecekan karena akan berdampak kepada komponen yang lain yang sudah terintegrasi.
-	Kompleksitas code menjadi lebih sudah dan besar.
-	Monolit code susah untuk dirawat, susah untuk membuat code tetap bersih dan rapi.
-	Membutuhkan spesifikasi perangkat keras yang lebih tinggi untuk melakukan komputasi lebih besar dalam proses scaling up.
-	Proses scaling up akan lebih lama.

## Links

- https://www.atlassian.com/microservices/microservices-architecture/microservices-vs-monolith
- https://www.digitalocean.com/blog/monolithic-vs-microservice-architecture
- https://azure.microsoft.com/en-gb/products/service-fabric
- https://www.ibm.com/topics/microservices
- https://www.atlassian.com/microservices/microservices-architecture

## Notes

- Author: Aditya Kristianto
- Version: 0.1
- Changelog:
    - 0.1: versi pengajuan awal