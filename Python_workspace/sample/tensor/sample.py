import tensorflow as tf
with tf.compat.v1.Session() as sess:
 hello = tf.constant('Hello, TensorFlow!')
 sess = tf.compat.v1.Session()
 result = sess.run(hello)
 print(result)