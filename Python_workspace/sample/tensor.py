import sys
import tensorflow as tf
tf.compat.v1.disable_eager_execution()
sys.stderr.write("*** start ***\n")

input_x = [[1.],[5.]]
input_y = [[4.],[2.]]

x = tf.compat.v1.placeholder("float", [None, 1])
y_ = tf.compat.v1.placeholder("float", [None, 1])

a = tf.Variable([1.], name="slope")
b = tf.Variable([0.], name="y-intercept")
y = tf.multiply(a, x) + b

init = tf.compat.v1.global_variables_initializer()

# 誤差関数
loss = tf.reduce_sum(tf.square(y_ - y))

# トレーニング方法は、勾配降下法を選択
train_step = tf.compat.v1.train.GradientDescentOptimizer(0.03).minimize(loss)


with tf.compat.v1.Session() as sess:
    sess.run(init)
    print('Initial State')
    print('Error' + str(sess.run(loss, feed_dict={x: input_x, y_: input_y})))
    print("slope: %f, y-intercept: %f" % (sess.run(a), sess.run(b)))

    for step in range(100):
        sess.run(train_step, feed_dict={x: input_x, y_: input_y})
        if (step+1) % 20 == 0:
            print('\nStep: %s' % (step+1))
            print('Error' + str(sess.run(loss, feed_dict={x: input_x, y_: input_y})))
            print("slope: %f, y-intercept: %f" % (sess.run(a), sess.run(b)))
#
sys.stderr.write("*** end ***\n")