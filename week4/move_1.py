#!/usr/bin/env python3

import sys
import rospy
import moveit_commander
from geometry_msgs.msg import Pose

def main():
    # Initialize the moveit_commander and rospy
    moveit_commander.roscpp_initialize(sys.argv)
    rospy.init_node('move_1', anonymous=True)

    # Instantiate a `RobotCommander` object. Provides information about the robot's kinematic model and current state
    robot = moveit_commander.RobotCommander()

    # Instantiate a `PlanningSceneInterface` object. Allows control of the environment
    scene = moveit_commander.PlanningSceneInterface()

    # Instantiate a `MoveGroupCommander` object. This object is an interface to a planning group (group of joints)
    group_name = "panda_arm"
    move_group = moveit_commander.MoveGroupCommander(group_name)

    # Define a target pose
    pose_goal = Pose()
    pose_goal.orientation.w = 1.0
    pose_goal.position.x = 0.4
    pose_goal.position.y = 0.1
    pose_goal.position.z = 0.4
    move_group.set_pose_target(pose_goal)

    # Plan and execute the motion
    plan = move_group.go(wait=True)

    # Ensure no residual targets
    move_group.stop()
    move_group.clear_pose_targets()

    # Shutdown
    moveit_commander.roscpp_shutdown()

if __name__ == '__main__':
    try:
        main()
    except rospy.ROSInterruptException:
        pass
