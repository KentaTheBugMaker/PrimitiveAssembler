 
The state required to support triangle strips consists of a flag indicating
if the first triangle has been completed, two stored processed vertices, (called
vertex A and vertex B), and a one bit pointer indicating which stored vertex
will be replaced with the next vertex.


トライアングルストリップをサポートするためには次のような状態変数が必要とされます。
    - bool 最初の三角形が完成したか
    - VertexA 処理された頂点
    - VertexB 処理された頂点
    - ReplaceTarget 次に置き換えられる頂点(A or B)
enum ReplaceTarget{
A, 
B
}
Begin(TRIANGLE_STRIP) のあとReplaceTargetはvertexAに初期化されます。

After a Begin(TRIANGLE STRIP),the pointer is initialized to point to vertex A.
Begin / Endに囲われたそれぞれの頂点は ReplaceTarget を切り替えます
Each vertex sent between a　Begin/End pair toggles the pointer.
以上のことから最初の頂点はVertexAに2番目はVertexB 3番めはVetexA以下はそのようにする 
Therefore, the first vertex is stored as　vertex A, the second stored as vertex B, the third stored as vertex A, and　so on.
どのような2番め以降に送られた頂点は　VertexA,VertexB 現在の頂点で三角形を構成します。
Any vertex after the second one sent forms a triangle from vertex A,vertex B, and the current vertex (in that order).

