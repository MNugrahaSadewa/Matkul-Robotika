#!/user/bin/env python3
import rospy

if _name_ == '_main_':
    rospy.init_node("tes_node")
    rospy.loginfo("Hello form Test node")
    
    rate = rospy.Rate(10)

    while not rospy.is_shutdown():
        rospy.loginfo("HelOOOOOOOOOOOOOOOOOOOOO")
        rate.sleep()