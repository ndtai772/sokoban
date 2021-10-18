### Luật chơi được chuyển đổi sang hướng *programable*:
* Các đối tượng W(wall), P(player), E(empty), ET(empty-target), B(box), BT(boxed-target)
* Số box bằng với số target, game mặc định là solvable
* Player chỉ có thể di chuyển bằng các hành động move (ML-MR-MU-MD), push (PL-PR-PU-PD)
* Box chỉ có thể di chuyển khi chịu tác động của Player
* Box chỉ có thể di chuyển đến các vị trí E hoặc ET lân cận
* Khi một box được di chuyển đến vị trí ET nó trở thành BT, ngược lại nếu box bị di chuyển ra khỏi BT nó trở thành ET
* Player chỉ có thể di chuyển:
    - đến các vị trí E hoặc ET
    - các vị trí của box phía trước (B, BT) nếu box đó có thể tịnh tiến theo hướng từ player -> box
* Khi player di chuyển đến vị trí của một box thì box đó đồng thời tiến về phía trước cùng chiều với player (bị đẩy)
* Người chơi thua khi có ít nhất 1 box (B not BT) không thể di chuyển
* Người chơi thắng khi tất cả các target(ET) được 'đóng lại' (trở thành BT - boxed-target)

### Tóm tắt bài toán:
- Đầu vào là một *map* chứa vị trí của wall, player, empty, target, box
- Mỗi hành động di chuyển của người chơi được xem là một step - 1 node của cây hành động, chúng ta cần tìm tập các step phù hợp để biến đổi map ban đầu sang map có trạng thái win

### Phân tích, thiết kế giải thuật:
- Dễ thấy kể cả những game 'dễ' cũng cần từ 30-40 bước di chuyển để hoàn thành trong điều kiện tốt nhất -> khi đó ta cần xét tối đa 4^30 ~1.15e18 trường hợp -> sử dụng DFS nhằm tránh tràn bộ nhớ, thay vì lưu lại toàn bộ map với mỗi step ta tạo ra các bước undo với mỗi loại hành động của 'player'
- Nếu người chơi ML rồi sau đó MR (hoặc MU-MD, MR-ML ...) thì game lại trở về trạng thái ban đầu -> sử dụng biện pháp cắt cạnh để giảm những trường hợp này khi backtracking
- Các hành động move sẽ không làm thay đổi trạng thái của game, do đó thay vì di chuyển một cách 'ngẫu nhiên' cho đến khi tìm được phương án phù hợp ta chỉ quan tâm đến các bước push
