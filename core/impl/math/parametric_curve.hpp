#ifndef E2_PARAMETRICCURVE_HPP
#define E2_PARAMETRICCURVE_HPP

namespace e2 { namespace math {

///provides ability to trace out position of a cubic bezier curve using forward differencing. Taken from pp.365-367 of Chapter 8 of Advanced 3D Game Programming with DirectX 10.0 by Peter Walsh
class parametric_curve
{
 private:
  /// current step
  int mCurrentStep;

  /// total steps for this curve
  int mTotalStep;

  /// bezier control points
  float mControlPoints[4][3];
  
  /// current position and derivatives' positions
  float mPoint[3];
  float mDPoint[3];
  float mDDPoint[3];
  float mDDDPoint[3];

  /// bezier basis matrix
  float mBezierBasis[4][4] = {{-1,3,-3,1}, {3,-6,3,0}, {-3,3,0,0}, {1,0,0,0}};

  bool bStarted;

 public:
  
  parametric_curve();

  ///sets control points for the curve
  void set_parameter(int steps, float control1[], float control2[], float control3[], float control4[]);

  /// go to next step of the curve
  void increment();

  /// returns current position
  void get_current(float*& out); 

  ///initialize the curve
  void start();

  ///see if the curve had reached the end
  bool done();

  bool started();
};

} }

#endif
