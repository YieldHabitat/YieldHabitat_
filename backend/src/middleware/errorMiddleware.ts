import { Request, Response, NextFunction } from 'express';

// Error handling middleware
export const errorHandler = (
  err: Error,
  req: Request,
  res: Response,
  next: NextFunction
) => {
  console.error(`Error: ${err.message}`);
  console.error(err.stack);
  
  res.status(500).json({
    message: 'Server Error',
    stack: process.env.NODE_ENV === 'production' ? null : err.stack,
  });
}; 